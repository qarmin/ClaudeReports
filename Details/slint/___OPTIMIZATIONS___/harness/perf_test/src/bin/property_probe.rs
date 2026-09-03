//! What a property read costs.
//!
//! Every traversal in this workspace is priced in "ns per item", and every one of those figures is
//! really a count of property reads: `ItemRc::geometry()` is four of them, and the ~90-124 ns per
//! item visit measured by candidates 15, 18 and 33 is a handful more. That constant had only ever
//! been inferred by division. This measures it.

use std::pin::Pin;
use std::time::Instant;

use i_slint_core::SharedString;
use i_slint_core::callbacks::Callback;
use i_slint_core::properties::{Property, PropertyTracker};

const N: usize = 2_000_000;

fn bench(label: &str, f: impl FnOnce() -> f64) {
    let ns = f();
    println!("{label:<46} {ns:>8.2} ns");
}

fn main() {
    // A property with no binding: the cheapest possible read.
    let plain = Box::pin(Property::new(1.0f32));
    bench("plain get, no tracker", || {
        let start = Instant::now();
        let mut acc = 0.0f32;
        for _ in 0..N {
            acc += plain.as_ref().get();
        }
        std::hint::black_box(acc);
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });

    // A property whose value comes from a binding, already evaluated and clean.
    let source = Box::pin(Property::new(2.0f32));
    let derived: Pin<Box<Property<f32>>> = Box::pin(Property::default());
    // SAFETY: `source` is pinned in a `Box` that outlives `derived`, and is never moved.
    let source_ref: Pin<&'static Property<f32>> =
        unsafe { std::mem::transmute::<Pin<&Property<f32>>, Pin<&'static Property<f32>>>(source.as_ref()) };
    derived.as_ref().set_binding(move || source_ref.get() * 2.0);
    let _ = derived.as_ref().get();
    bench("clean binding get, no tracker", || {
        let start = Instant::now();
        let mut acc = 0.0f32;
        for _ in 0..N {
            acc += derived.as_ref().get();
        }
        std::hint::black_box(acc);
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });

    let tracker: Pin<Box<PropertyTracker>> = Box::pin(PropertyTracker::default());

    // Reading the *same* property a million times inside one tracker registers the dependency once:
    // candidate 08's `front_is` short-circuit takes every read after the first. That is not what a
    // traversal does, so it is measured separately from the case that matters.
    bench("same property re-read inside a tracker", || {
        let start = Instant::now();
        tracker.as_ref().evaluate(|| {
            let mut acc = 0.0f32;
            for _ in 0..N {
                acc += derived.as_ref().get();
            }
            std::hint::black_box(acc);
        });
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });

    // What a traversal actually does: a different property on every read, so every one of them
    // registers a dependency node on the tracker.
    const DISTINCT: usize = 100_000;
    let many: Vec<Pin<Box<Property<f32>>>> =
        (0..DISTINCT).map(|i| Box::pin(Property::new(i as f32))).collect();

    bench("distinct properties, no tracker", || {
        let start = Instant::now();
        let mut acc = 0.0f32;
        for _ in 0..(N / DISTINCT) {
            for p in &many {
                acc += p.as_ref().get();
            }
        }
        std::hint::black_box(acc);
        start.elapsed().as_secs_f64() * 1e9 / ((N / DISTINCT) * DISTINCT) as f64
    });

    bench("distinct properties, inside a tracker", || {
        let start = Instant::now();
        for _ in 0..(N / DISTINCT) {
            tracker.as_ref().evaluate(|| {
                let mut acc = 0.0f32;
                for p in &many {
                    acc += p.as_ref().get();
                }
                std::hint::black_box(acc);
            });
        }
        start.elapsed().as_secs_f64() * 1e9 / ((N / DISTINCT) * DISTINCT) as f64
    });

    // A tracker evaluated once per read, which is the shape of a per-item tracker in the renderer.
    bench("one tracker evaluation per read", || {
        let start = Instant::now();
        for _ in 0..(N / 20) {
            tracker.as_ref().evaluate(|| {
                std::hint::black_box(derived.as_ref().get());
            });
        }
        start.elapsed().as_secs_f64() * 1e9 / (N / 20) as f64
    });

    // A `string` property. Real UIs are full of these - a row's label, a formatted number - and
    // `SharedString` is refcounted, so a read is a refcount bump rather than a copy. Nothing here had
    // measured what that costs against a `f32`.
    let text: Pin<Box<Property<SharedString>>> =
        Box::pin(Property::new(SharedString::from("item 000123")));
    bench("SharedString get (refcount bump)", || {
        let start = Instant::now();
        let mut len = 0usize;
        for _ in 0..N {
            len += text.as_ref().get().len();
        }
        std::hint::black_box(len);
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });

    // The binding a row's label usually is: build a string from a number every time it is dirty.
    let index: Pin<Box<Property<i32>>> = Box::pin(Property::new(0));
    let label: Pin<Box<Property<SharedString>>> = Box::pin(Property::default());
    // SAFETY: `index` is pinned in a `Box` that outlives `label`, and is never moved.
    let index_ref: Pin<&'static Property<i32>> =
        unsafe { std::mem::transmute::<Pin<&Property<i32>>, Pin<&'static Property<i32>>>(index.as_ref()) };
    label.as_ref().set_binding(move || SharedString::from(format!("item {:06}", index_ref.get())));
    let _ = label.as_ref().get();
    bench("formatted string binding, clean", || {
        let start = Instant::now();
        let mut len = 0usize;
        for _ in 0..N {
            len += label.as_ref().get().len();
        }
        std::hint::black_box(len);
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });
    bench("formatted string binding, re-evaluated", || {
        let start = Instant::now();
        let mut len = 0usize;
        for i in 0..(N / 20) {
            index.as_ref().set(i as i32);
            len += label.as_ref().get().len();
        }
        std::hint::black_box(len);
        start.elapsed().as_secs_f64() * 1e9 / (N / 20) as f64
    });

    // A callback invocation. Every clickable row in a real UI has one, and `Callback` stores its
    // handler in a `Cell<Option<Box<dyn FnMut>>>` - so a call takes the box out, runs it and puts it
    // back. Nothing here had measured that against a property read.
    let callback: Callback<(), i32> = Callback::default();
    callback.set_handler(|()| 7);
    bench("callback call", || {
        let start = Instant::now();
        let mut acc = 0i32;
        for _ in 0..N {
            acc += std::hint::black_box(&callback).call(&());
        }
        std::hint::black_box(acc);
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });

    // The control has to be opaque to the optimizer, or it folds to nothing and reads as 0.00 ns -
    // which is what the first version of this did.
    let plain_closure: Box<dyn Fn() -> i32> = Box::new(|| 7);
    bench("plain boxed closure call (control)", || {
        let start = Instant::now();
        let mut acc = 0i32;
        for _ in 0..N {
            acc += std::hint::black_box(&plain_closure)();
        }
        std::hint::black_box(acc);
        start.elapsed().as_secs_f64() * 1e9 / N as f64
    });

    // Writing the source dirties the derived property, so every read re-evaluates the binding.
    bench("get after dirtying (binding re-evaluated)", || {
        let start = Instant::now();
        let mut acc = 0.0f32;
        for i in 0..(N / 20) {
            source.as_ref().set(i as f32);
            acc += derived.as_ref().get();
        }
        std::hint::black_box(acc);
        start.elapsed().as_secs_f64() * 1e9 / (N / 20) as f64
    });
}
