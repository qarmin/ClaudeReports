# Model wątkowości w Godocie - notatki

Nie jest to candidate - to wyjaśnienie modelu wątkowości silnika napisane w trakcie sesji
bug-huntu (2026-10-07), żeby ułatwić zrozumienie *czemu* tak wiele kandydatów w tej sesji ma
identyczny kształt (brak locka / brak sprawdzenia null po thread-guardzie). Trzymane osobno od
`SUMMARY.md`/`IDEAS.md`, bo to wiedza kontekstowa, nie wynik konkretnego kandydata.

## Model w skrócie

W Godocie równolegle działa kilka kategorii wątków, które mogą dotykać tych samych obiektów
silnika w tym samym czasie:

- **main thread** - jedyny, któremu normalnie wolno modyfikować `SceneTree`/`Node` (dodawanie/
  usuwanie dzieci, sygnały, itp.),
- **wątek renderowania** (gdy włączone "rendering thread" / RenderingServer w wątku),
- **wątek fizyki** (gdy `physics/2d|3d/run_on_separate_thread` jest włączone -
  `PhysicsServer2DWrapMT`/`3D` marszalują wywołania przez `CommandQueueMT`),
- **`WorkerThreadPool`** - pool wątków używany przez GDScript (`Thread`, zadania równoległe,
  import assetów itd.), w tym "thread groups" dla `Node.process_thread_group`,
- **wątki natywne stworzone przez moduły** - np. `CameraLinux`'s polling thread,
  `RemoteDebuggerPeerTCP`'s poll/read thread.

Każdy z tych wątków może wywołać publiczne API klasy silnika (singleton typu
`Input`/`CameraServer`/`ProjectSettings`/`AudioServer`, albo zasób typu `Curve`/`Image`/`BitMap`)
w tym samym czasie co inny wątek. To jest powód istnienia dwóch różnych mechanizmów ochrony, które
w tej sesji bug-huntu wielokrotnie okazały się dziurawe.

## Mechanizm 1: `_THREAD_SAFE_CLASS_` / `_THREAD_SAFE_METHOD_`

Definicja, `core/os/thread_safe.h:35-38`:

```cpp
#define _THREAD_SAFE_CLASS_ mutable Mutex _thread_safe_;
#define _THREAD_SAFE_METHOD_ MutexLock _thread_safe_method_(_thread_safe_);
```

Zwykły, **ręczny** idiom RAII: `_THREAD_SAFE_CLASS_` dodaje do klasy pole `Mutex`, a
`_THREAD_SAFE_METHOD_` wstawiony na początku metody blokuje go na czas jej działania (destruktor
`MutexLock` odblokowuje przy wyjściu, nawet przy wczesnym `return`). Stosuje się go w klasach, które
mają być bezpiecznie współdzielone między wątkami - serwery (`AudioServer`, `CameraServer`),
singletony (`Input`, `InputMap`, `ProjectSettings`) i proste zasoby (`Curve`, `Curve2D`/`Curve3D`).

**Problem: to czysto konwencyjny mechanizm, kompilator nic nie wymusza.** `_THREAD_SAFE_CLASS_`
dodaje tylko *pole* - nic nie sprawdza, czy dana metoda rzeczywiście bierze lock przed dotknięciem
chronionych danych. Znalezione w tej sesji realne konsekwencje:

- **candidate 009** (`CameraServer`) - klasa deklaruje `_THREAD_SAFE_CLASS_`, ale żadna z metod
  dotykających `feeds` faktycznie nie woła `_THREAD_SAFE_METHOD_` - mutex istnieje, ale jest
  "martwy".
- **candidate 010/029** (`ProjectSettings`) - część metod (`_set`/`_get`) lockuje, ale
  `clear()`/`get_order()`/`set_custom_property_info()` nie - klasyczna asymetria "writer lockuje,
  reader nie", więc i tak nie ma wzajemnego wykluczenia.
- **candidate 012** (`InputMap`) - zero locków w całej klasie od 2014 roku (`0b806ee0fc`).
- **candidate 018/030** (`PopupMenu`) - tylko 2 z ~100 publicznych metod dotykających `items` mają
  lock.

Czyli: to że klasa *ma* mutex, albo że *niektóre* metody go używają, nic nie mówi o pozostałych.
Kompilator C++ nie wie, że pole `_thread_safe_` powinno chronić pole `feeds`/`items`/`props` - to
wiedza tylko w głowie autora patcha.

## Mechanizm 2: `ERR_READ_THREAD_GUARD` / `ERR_MAIN_THREAD_GUARD`

Zupełnie inny mechanizm, dla `Node`/`SceneTree` (`scene/main/node.h:937-947`):

```cpp
#define ERR_MAIN_THREAD_GUARD ERR_FAIL_COND_MSG(is_inside_tree() && !is_current_thread_safe_for_nodes(), ...);
#define ERR_READ_THREAD_GUARD ERR_FAIL_COND_MSG(!is_readable_from_caller_thread(), ...);
#define ERR_READ_THREAD_GUARD_V(m_ret) ERR_FAIL_COND_V_MSG(!is_readable_from_caller_thread(), (m_ret), ...);
```

`SceneTree`/`Node` **nie są** chronione lockiem - drzewo scenowe jest z założenia single-threaded
(zbyt kosztowne byłoby lockować je przy każdym `add_child`/sygnale). Zamiast tego te makra to
**runtime-check** (`ERR_FAIL_COND_MSG`/`ERR_FAIL_COND_V_MSG`) sprawdzający, czy wołający wątek to
main thread (albo dozwolona thread group) - jeśli nie, loguje błąd i **zwraca wartość domyślną**
(`m_ret`, np. `nullptr`) zamiast przejść dalej. Zalecenie dla nie-main-threadów to
`call_deferred()`, żeby zmarszalować pracę na main thread.

To jednak tylko *jedna linijka ochrony w jednym miejscu* - jeśli funkcja wywołująca ten getter i
odrazu dereferencjonuje wynik bez sprawdzenia null, guard i tak nie pomoże. Dokładnie to był
najczęstszy wzorzec bugów w tej sesji - rodzina kandydatów **014/017/019-026/031-033**:
`get_world_3d()`/`get_window()` poprawnie zwraca `nullptr` z zagranicznego wątku, ale kod wołający
(`get_navigation_map()`, `show_popup()`, `_update_debug_shape()` itd.) dereferencjonuje ten wynik
bez sprawdzenia - deterministyczny crash, bo guard chroni *jedną funkcję*, a nie *cały łańcuch
wywołań*.

## Czy da się to wymusić statycznie (przez kompilator/typy)?

Krótko: w C++ częściowo tak, ale nic z tego nie jest "włącz i zapomnij" tak jak w Rust. Realne
opcje:

1. **Clang Thread Safety Analysis** (`-Wthread-safety`, atrybuty
   `GUARDED_BY`/`REQUIRES`/`CAPABILITY`, znane z Abseil/Chromium). Teoretycznie dałoby się oznaczyć
   `Mutex` jako `CAPABILITY`, a chronione pola jako `GUARDED_BY(_thread_safe_)` - wtedy Clang przy
   kompilacji wykryłby dostęp do pola bez trzymanego locka (złapałby np. cały bug 009/012/018/030
   na etapie kompilacji). Ograniczenia: działa tylko pod Clang (Godot wspiera też GCC/MSVC),
   wymaga adnotacji *każdego* chronionego pola w *każdej* klasie z osobna - przy setkach klas to
   ogromny, rozłożony w czasie refaktor, nie przełącznik.

2. **Enkapsulacja typu "Rust `Mutex<T>`" zamiast "pole + osobny mutex"** - to jest właściwie to,
   co już zrobiono w realnym commicie `b8420aa3a6` ("Fix race... via a Rust style `OwnedMutex`"):
   owija się dane *w* wrapper, z którego nie da się wyciągnąć surowej referencji bez przejścia
   przez `.lock()`. To eliminuje całą klasę bugów "zapomniałem dopisać `_THREAD_SAFE_METHOD_` w
   tej jednej metodzie", bo kompilator (przez prywatność/typy, nie konwencję) nie pozwoli dotknąć
   danych inaczej. Realistyczna droga długoterminowa, ale wymaga przepisania każdej klasy z
   osobna - nie jest globalnym przełącznikiem.

3. **`clang-tidy`/`cppcheck --enable=thread`** - łapią proste przypadki (brak locka w oczywistym
   miejscu), ale przy wzorcu Godota (lock w części metod, nie w innych, bez żadnych adnotacji)
   heurystyki mają wysoki wskaźnik szumu/przeoczeń - nie zastępują realnej analizy.

4. **ThreadSanitizer w CI, na testach faktycznie wielowątkowych** - jedyny mechanizm, który łapie
   *rzeczywiste* races w runtime (użyty już w candidate 028, TSan na prawdziwym repro TCP).
   Prawdziwy powód, czemu te ~30 bugów przetrwało latami, to brak takich testów w CI Godota -
   guard/mutex to tylko *intencja*, bez testu stresującego wielowątkowość nikt nie wie, że
   intencja się nie trzyma.

**Podsumowanie:** pełne wymuszenie "każda klasa dostępna z innego wątku musi być bezpieczna,
sprawdzane przez kompilator" nie jest w C++ realistyczne jako jednorazowa zmiana - wymagałoby
połączenia (a) migracji współdzielonych danych do enkapsulujących wrapperów typu `OwnedMutex<T>`
per-klasa (to już się dzieje punktowo), (b) opcjonalnego `-Wthread-safety` pod Clangiem jako
dodatkowy check w CI z adnotacjami na kluczowych serwerach/singletonach, i (c) testów pod TSan w
CI dla realnych scenariuszy wielowątkowych. Samo przejście na coś w stylu Rusta usunęłoby problem
fundamentalnie, ale to nie jest coś, co się "włącza" w istniejącym C++ engine.
