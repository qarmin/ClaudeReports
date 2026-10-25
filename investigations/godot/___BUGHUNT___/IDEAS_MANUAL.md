# Pomysły na poprawki "przenieś check z głowy programisty na kompilator"

Do samodzielnego przeglądu - nic z tego nie zostało jeszcze zaimplementowane. Każdy punkt ma
konkretne file:line z kodu, nie jest teoretyczny, o ile nie zaznaczono inaczej.

Wzorzec odniesienia: `core/templates/owned_mutex.h` + `core/io/resource_uid.h`/`.cpp` (już
zaimplementowane w tej sesji, branch `owned_mutex`) - mutex i wartość są razem, więc nie da się
skompilować dostępu do wartości bez locka.

## TL;DR - priorytety

**Najważniejsze odkrycie całej sesji (Sekcja 18):** pobrano realne, zamknięte issues z
`godotengine/godot` (GitHub) i potwierdzono, że bug class znaleziony w tej sesji przez czytanie
kodu **regularnie crashuje użytkowników w produkcji** - nie jest teoretyczny (#118450 AudioServer,
#114896 MaterialStorage, #113210 Variant lazy-init, #63461 ResourceLoader - wszystkie ten sam
wzorzec co `ResourceUID`/`CallQueue`). Odkryto też, że **Clang Thread Safety Analysis
(`-Wthread-safety`, `GUARDED_BY`/`ACQUIRED_BEFORE`) jest całkowicie nieużywane w Godocie** - to
mechanizm, który złapałby każdy z tych bugów na etapie kompilacji, systemowo, nie punktowo.

Najmocniejsze, najbardziej "gotowe do zrobienia" pozycje (konkretny bug/wzorzec, mały zasięg):

1. **`CallQueue` (`core/object/message_queue.h`)** - ten sam bug co `ResourceUID::get_path_id()`,
   tylko `OwnedMutex` wystarczy. Sekcja 1.
2. **`Tween` (`scene/animation/tween.h`)** - 5 niezależnych booleanów → jeden `enum class`, ma
   precedens we własnym kodzie (`ResourceLoader::ThreadLoadStatus`). Sekcja 5.
3. **`FileAccessCompressed` (`core/io/file_access_compressed.cpp`)** - 8x powtórzony runtime-check
   "czy otwarte" → typestate. Sekcja 7.
4. **`RemoteDebuggerPeerTCP`** - realny brak synchronizacji na `connected`, nie tylko zapomniany
   lock. Sekcja 1.

Resztę (`[[nodiscard]]` na `Error`, mocne typy ID, `switch` bez exhaustiveness, `Span<T>`) warto
zrobić punktowo/etapowo, nie jako jeden duży PR - szczegóły w odpowiednich sekcjach, każda ma
własny werdykt na końcu.

**Zbadane i odrzucone (żeby nie tracić czasu na to samo dwa razy):** generalizacja
`ERR_MAIN_THREAD_GUARD` samego w sobie - jest zbyt specyficzna dla Node, nie da się jej wyciągnąć.
Zamiast tego: dwie małe, konkretne poprawki w sekcji 8 (makro `ERR_FAIL_NOT_MAIN_THREAD_MSG` na 42
miejsca + debug-only warning w `WorkerThreadPool::add_task()` gdy `Callable` jest związany z `Node`).

5. **`RBMap`/`RBSet::find()`** (`core/templates/rb_map.h:627,636`, `rb_set.h:578,587`) - brakująca
   adnotacja `_LIFETIME_BOUND_`, 4 linijki, dokańcza już istniejącą konwencję. Sekcja 9.
6. **Makro `ERR_FAIL_NOT_MAIN_THREAD_MSG`** - zamiennik 42 ręcznie pisanych miejsc +
   debug-warning w `WorkerThreadPool::add_task()` dla `Callable` związanych z `Node`. Sekcja 8.
7. **`-Woverloaded-virtual`** - zweryfikowane **pełnym buildem całego silnika**: tylko 2 unikalne,
   realne hity w całym projekcie (`core/os/os.h:328`, `editor/scene/2d/abstract_polygon_2d_editor.h:142`),
   zero szumu. Najlepszy stosunek nakład/efekt z całej listy. Sekcja 11.
8. **`RendererSceneCull::Instance`** (`servers/rendering/renderer_scene_cull.h:613`) - realny,
   niechroniony przypadkiem double-free przy skopiowaniu (2 gołe pointery w destruktorze, zero
   `=delete`). Sekcja 12.
9. **`BitField<T>`** - `Skeleton3D::update_flags` (`scene/3d/skeleton_3d.h`) i
   `BaseMaterial3D::stencil_flags` (`scene/resources/material.h:623`) - enum i typ już istnieją,
   trzeba tylko zmienić typ pola. Sekcja 13.

**Korekta po pełnym buildzie:** `-Wsuggest-override` (pkt 7 w poprzedniej wersji tej listy) okazał
się dużo większy niż pierwsza próbka sugerowała - **385 unikalnych miejsc w ~14 plikach**, nie 9
linii. Wciąż warto, ale to średnia poprawka (backendy renderera systematycznie nie mają
`override`), nie mała - zob. Sekcja 11.

## 1. Więcej kandydatów na `OwnedMutex` (ten sam kształt buga co w `ResourceUID`)

### Solidne, potwierdzone (locked-niektóre/unlocked-inne, ten sam wzorzec)

**`core/object/message_queue.h`/`.cpp` - `CallQueue`** (kolejka odłożonych wywołań silnika, bardzo
gorący kod). `mutex` chroni `pages`/`page_bytes`/`pages_used`/`flushing`, wszystkie mutatory
(`push_callp`, `flush`, `_add_page`) używają `LOCK_MUTEX`/`UNLOCK_MUTEX`. Trzy publiczne akcesory
czytają te same pola **bez żadnego locka**:
- `is_flushing()` (`message_queue.cpp:441`) - czyta `flushing`
- `has_messages()` (`message_queue.cpp:445-453`) - czyta `pages_used`, `page_bytes`
- `get_max_buffer_usage()` (`message_queue.cpp:456-458`) - czyta `pages`

**`core/debugger/remote_debugger_peer.h`/`.cpp` - `RemoteDebuggerPeerTCP`**. `mutex` chroni
`in_queue`/`out_queue` (poprawnie w `get_message()`/`put_message()`), ale:
- `has_message()` (linia 44-46) czyta `in_queue.size()` bez locka
- `connected` (goły `bool`) jest pisany bez synchronizacji przez wątek pollujący (`_poll()`, linia
  219-222) i czytany bez synchronizacji przez `is_peer_connected()` (linia 40-42) - to pole w ogóle
  nigdy nie było pod żadną ochroną, nie "zapomniany lock" tylko kompletny brak.

### Słabszy, niepewny przypadek - wymaga wiedzy domenowej przed zgłoszeniem jako bug

**`scene/gui/menu_bar.h`/`.cpp` - `MenuBar::menu_cache`**. Lockowane tylko w 2 z ~12 metod, które
dotykają `menu_cache` (`gui_input`, `NOTIFICATION_INTERNAL_PROCESS`) - `NOTIFICATION_DRAW`, sync z
natywnym menu, `shape()` i inne (~10+ miejsc) dotykają bez locka. Może być celowe (wąski lock pod
konkretny callback z innego wątku), nie wygląda jak prosty przeoczony przypadek.

### Sprawdzone i wykluczone (dyscyplina locka spójna)

- `core/io/ip.cpp` (`_IP_ResolverPrivate`) - świadomy split: `SafeNumeric<status>` bez locka,
  reszta pod lockiem wszędzie.
- `scene/gui/rich_text_label.h` (`data_mutex`) - 30+ konsekwentnych `MutexLock`.
- `modules/gdscript/gdscript.h` (`mutex`, `func_ptrs_to_update_mutex`) - pervazywnie lockowane
  (20+ miejsc).
- `core/templates/rid_owner.h` (`RID_Alloc`) - świadomie zaprojektowany hybryd lock-free/locked
  (`if constexpr (THREAD_SAFE)` + jawne atomic store z komentarzem o unikaniu data race) - nie
  dotykać.

### Nieprzejrzane w tej sesji (nie znaczy że czyste - do zrobienia później)

`modules/mono/csharp_script.h`, `modules/navigation_3d/3d/*`, `modules/navigation_2d/2d/*`,
`servers/rendering/renderer_rd/*`, `modules/jolt_physics/*`, `modules/text_server_adv/*`,
`modules/text_server_fb/*`, `core/math/bvh.h`, `modules/camera/*`.

**Rekomendacja:** `CallQueue` (`message_queue.h`) to naturalny kolejny `OwnedMutex` - ten sam
kształt buga, w kodzie dotykanym przez cały silnik.

## 2. `[[nodiscard]]` na `Error` - realny zysk, ale głośny

`enum Error` (`core/error/error_list.h:46`) nie ma `[[nodiscard]]`. Jedna linia
(`enum [[nodiscard]] Error { ... }`) wymusiłaby ostrzeżenie kompilatora na każdym zignorowanym
zwróconym `Error` w całym silniku - ale to ok. **1316 miejsc w 129 plikach** (core/+servers/+scene/).

Realne, dziś ignorowane zwroty (nie hipotetyczne):
- `core/io/resource_uid.cpp:289` - `dir->make_dir_recursive(...)` (ten sam plik co właśnie
  patchowaliśmy), zwrot zignorowany (w praktyce zamaskowany przez następny `FileAccess::open()`
  null-check, ale wciąż ignorowany `Error`)
- `core/io/logger.cpp:134,159` - `da->remove(...)`, `da->make_dir_recursive(...)`, gołe statementy
- `core/io/resource_format_binary.cpp:1278,1426,2383,2437` - cztery `da->remove(p_path...)`
- `core/extension/gdextension_manager.cpp:406`, `scene/resources/resource_format_text.cpp:1574` -
  ten sam wzorzec
- `core/io/file_access.cpp:153`, `core/io/dir_access.cpp:398`, `core/os/os.cpp:382` -
  `DirAccess::remove_absolute(...)`, wyraźnie best-effort cleanup plików tymczasowych/lockfile

`ERR_FAIL_COND_V`-makra (`core/error/error_macros.h`) nie mają żadnego związku z `[[nodiscard]]` -
reagują tylko na warunek, który caller sam napisał; jeśli caller nigdy nie spojrzy na zwrot, makra
nic nie zrobią. Sprawdzanie dziś to 100% dyscyplina programisty.

**Werdykt: realny zysk, ale głośny.** Nie flipować enuma na raz (zalałoby to ~1316 ostrzeżeniami
niezwiązany PR, część to świadome fire-and-forget cleanupy, gdzie błąd jest nieszkodliwy). Lepiej:
skompilować raz z `-Wunused-result`, przechodzić moduł po module, dodając `(void)`/`[[maybe_unused]]`
gdzie ignorowanie jest świadome. Precedens `[[nodiscard]]` na typach już istnieje w kodzie
(`core/math/{aabb,vector2,...}.h` używają `struct [[nodiscard]] X`), idiom się wpisuje w styl.

## 3. Mocne typy ID (jak `RID`) - wzorzec dobry, ale niski priorytet

`RID` (`core/templates/rid.h`) jest **w pełni opakowany** - brak niejawnej konwersji z/do
`uint64_t`, tylko explicit `RID::from_uint64()`/`.get_id()`. `ObjectID` jest "pół-mocny" celowo
(explicit konstruktor, ale niejawna konwersja na zewnątrz - bo musi współpracować z `Variant`,
chroni przed utratą precyzji, nie przed pomyleniem ID).

Inne gołe typedefy ID: `ResourceUID::ID` (`int64_t`, `resource_uid.h:45`),
`WorkerThreadPool::TaskID`/`GroupID` (`worker_thread_pool.h:51-52`, **ten sam** `int64_t` dla dwóch
różnych przestrzeni ID - ciekawszy przypadek), `IP::ResolverID` (`ip.h:64`, `int`),
`ProcessID` (`process_id.h:35`, `int64_t`, brak sentinela "invalid").

**Brak jakichkolwiek dowodów** (commity, TODO, historia buga) na realne pomylenie tych ID w
praktyce - to dziś czysto hipotetyczna ochrona typu. Owinięcie `ResourceUID::ID` dotyka **183
miejsc** w silniku (bindingi, serializacja, konwersje `Variant`) - wykonalne w stylu `RID`
(explicit fabryka + explicit `.get_id()`), ale duża, inwazyjna zmiana bez udokumentowanego buga
jako uzasadnienia.

**Werdykt: wzorzec wartościowy, ma precedens (`RID`), ale nie dla `ResourceUID::ID` teraz** - brak
dowodu na realny problem, wysoki koszt. `WorkerThreadPool::TaskID`/`GroupID` ciekawszy kandydat
(dwie przestrzenie ID pod tym samym typem), ale też bez potwierdzonego incydentu - "wart zrobienia
kiedyś", nie pilne.

---

## 4. Niewyczerpujące `switch` na własnych enumach (brak exhaustiveness jak w Rustowym `match`)

Rustowy `match` wymusza obsłużenie każdego warianta (albo jawny wildcard) - kompilator krzyczy, gdy
dodasz nowy wariant i gdzieś zapomnisz go obsłużyć. C++ `switch` może dać to samo, ALE tylko gdy
nie ma `default:` i jest włączone `-Wswitch` - `default:` zabija tę siatkę bezpieczeństwa: nowa
wartość enuma dodana później nie daje żadnego ostrzeżenia w miejscu, które powinno ją obsłużyć.

**Build:** `SConstruct:1006` ustawia `-Wall` (zawiera `-Wswitch`, ale to ostrzega tylko gdy NIE ma
`default:`). Nigdzie w `SConstruct` nie ma `-Wswitch-enum` (ostrzega nawet z `default:`
obecnym) - potwierdzone, że to nie przeoczenie w raporcie, tylko realny brak w konfiguracji builda.

**Realne znalezione przypadki (niebezpieczny wariant - `default` zwraca coś, co wygląda OK, ale
może być źle):**

- `core/os/midi_driver.cpp:79-94` - `MIDIDriver::Parser::expected_data(MIDIMessage)` zwraca 2 lub 1
  dla znanych wiadomości, `default: return 0`. Nowa wartość `MIDIMessage`, która faktycznie
  potrzebuje bajtu danych, dostałaby cicho `0` - wygląda płauzybilnie, jest błędne.
- `core/input/input_event_codec.cpp:395-420` - `switch` na `InputEventType` (12 wartości), tylko 7
  obsłużonych jawnie, SCREEN_TOUCH/SCREEN_DRAG/MIDI/SHORTCUT/ACTION w `default: return false;`.
  Może być celowe (te typy może nie powinny być kodowane sieciowo), ale nic nie wymusza, by ta
  decyzja była jawna/re-review'owana przy dodaniu nowego typu.

**Dla kontrastu - przykłady, gdzie `default` jest świadomy i dokumentowany (NIE są to findings):**
`core/io/image.cpp:2824-2826` (komentarz wyjaśnia dlaczego), `servers/rendering/rendering_device.cpp:95-117`
(bezpieczny fallback "Other"/najniższy priorytet).

**Werdykt: mieszany, warto poprawić selektywnie.** Większość `default:` jest świadoma. Rekomendacja:
usunąć `default:` (albo przejść na `-Wswitch-enum` punktowo, bo projektowo byłoby zbyt głośne) tylko
w konkretnych wysokiego-ryzyka miejscach jak `midi_driver.cpp`/`input_event_codec.cpp` powyżej, nie
jako kampania na cały kodebase.

## 5. "Zupa boolowska" zamiast jednego stanu-enuma (illegal states representable)

Rustowe enumy (sum types) robią niemożliwe-do-przedstawienia stany nielegalne - węzeł nie może być
jednocześnie "loading" i "loaded" i "failed", jeśli to jeden `enum class LoadState`, a nie trzy
niezależne booleany. Odwrotność tego w C++: klasa z 2+ polami `bool`, które są w praktyce fazami
jednego cyklu życia - nic nie stoi na przeszkodzie, by ustawić kilka naraz albo żadnego.

**Realny, mocny przypadek:** `scene/animation/tween.h:57,123-134` - `Tween` ma **5 niezależnych
booleanów** opisujących nakładające się fazy tego samego obiektu: `finished`, `started`, `running`,
`dead`, `valid`. Realny kod w `scene/animation/tween.cpp:211-213` ustawia 3 z nich ręcznie w jednym
miejscu: `running = false; valid = false; dead = true;` - trzy osobne przypisania, które muszą być
ręcznie zsynchronizowane; nic nie stoi na przeszkodzie, by przyszła edycja ustawiła tylko 2 z 3, albo
żeby skonstruować `Tween` z `dead=true, running=true` naraz (kombinacja logicznie bez sensu, ale typ
jej nie wyklucza). 2^5=32 reprezentowalnych kombinacji dla realistycznie ~4 prawdziwych stanów.

**Dobry kontrprzykład z tego samego projektu (precedens na właściwy kierunek fixu):**
`core/io/resource_loader.h:121-126,181-198` - `ResourceLoader::ThreadLoadTask` już używa
porządnego `enum ThreadLoadStatus { THREAD_LOAD_IN_PROGRESS, THREAD_LOAD_FAILED, THREAD_LOAD_LOADED, ... }`
zamiast boolowskich flag.

**Werdykt: warto poprawić.** `Tween` to mocny, konkretny kandydat na złączenie w jeden
`enum class TweenState { Idle, Running, Finished, Dead }` (czy podobny) - ta sama filozofia, którą
`ResourceLoader` już stosuje gdzie indziej w tym repo, więc fix ma precedens we własnym kodzie, nie
tylko zewnętrzną "rustową" motywację.

## 6. Adopcja `Span<T>` vs gołe pointer+length (bounds-safety jak w Rustowym `&[T]`)

Godot już ma `core/templates/span.h` - `Span<T>` to prawdziwy nie-właścicielski widok z długością
w komplecie (`Span(const T *p_ptr, uint64_t p_len)`), odpowiednik Rustowego `&[T]`: funkcja nie
może dostać pointera bez długości, i nie może przypadkiem użyć nieaktualnej/niezgodnej długości.

**Adopcja dziś:** 223 wystąpień w 43 plikach - skoncentrowane praktycznie wyłącznie w
`core/templates/` (cowdata, vector, local_vector, fixed_vector), `core/string/`, `core/variant/array`,
`servers/rendering/rendering_device*`. **Nie rozprzestrzeniło się** na szerszą powierzchnię API
pointer+length.

**Konkretni kandydaci na migrację** (gdzie mismatch długości to realny OOB, nie teoria):
- `core/io/marshalls.h:222` - `decode_variant(Variant&, const uint8_t *p_buffer, int p_len, ...)` -
  deserializuje **niezaufane dane z sieci/pliku** - klasyczny OOB read przy złej długości.
- `core/io/image.h:415` - `Image(const uint8_t *p_mem_png_jpg, int p_len=-1)` - parsuje niezaufane
  bajty obrazka z gołego pointera.
- `core/io/file_access.h:229` + 6 podklas (`file_access_{compressed,encrypted,memory,pack,patched,zip}.h`) -
  `store_buffer(const uint8_t *p_src, uint64_t p_length)`, wirtualny override powtórzony 6x.
- `core/io/net_socket.h:105-106` - `send`/`sendto(const uint8_t *p_buffer, int p_len, ...)`.
- `core/io/packet_peer.h:58` (+2 override) - `put_packet(const uint8_t *p_buffer, int p_buffer_size)`.

**Werdykt: mieszany, warto wybiórczo.** `decode_variant`/`Image`'s konstruktor to lepsze pierwsze
cele (pojedyncze call site, parsują niezaufane dane) niż wirtualne interfejsy jak `store_buffer`/
`NetSocket::send`, które wymagałyby ruszenia wszystkich override'ów naraz - to już większy refaktor.

## 7. "Musisz wywołać init()/open() przed use()" jako runtime bool, nie typ (typestate)

Rustowy typestate robi "użycie przed inicjalizacją" błędem kompilacji - stan niezainicjalizowany i
zainicjalizowany to różne typy, metoda `use()` po prostu nie istnieje na niezainicjalizowanym typie.
C++-owy odpowiednik tego buga: klasa z flagą `is_open`/`is_ready`, sprawdzaną w runtime przez każdą
metodę (`ERR_FAIL_COND(!is_open)`) - zapomnienie wywołania `open()`/`init()` jest wykrywane tylko w
runtime, nigdy w czasie kompilacji.

**Mocny, dobrze potwierdzony przypadek:** `core/io/file_access_compressed.cpp` -
`FileAccessCompressed` ma `Ref<FileAccess> f` jako de facto flagę gotowości, sprawdzaną przez
`ERR_FAIL_COND(_V)_MSG(f.is_null(), "File must be opened before use.")` w **8 osobnych metodach**:
`seek:190`, `seek_end:220`, `get_position:229`, `get_length:238`, `eof_reached:247`,
`get_buffer:261`, `flush:311`, `store_buffer:318`. Ten sam string literal powtórzony 8 razy -
podręcznikowy przypadek typestate (typ "otwarty" którego metody po prostu nie istnieją przed
`open()` zamiałby to w błąd kompilacji).

Nie znaleziono równie mocnego drugiego przykładu w czasie przeznaczonym na to badanie - to jeden
solidny, dobrze potwierdzony przypadek, nie jeszcze pokazany jako powszechny wzorzec w całym `core/io/`.

**Werdykt: warto zrobić dla tej jednej klasy, nie udowodnione jako szeroki problem.**
`FileAccessCompressed` sam uzasadnia refaktor (8 powtórzonych runtime-checków zwija się w jedną
gwarancję czasu kompilacji), ale nie generalizować na "większość `core/io/` ma ten problem" bez
sprawdzenia więcej klas niż czas pozwolił w tej sesji.

## 8. Rustowy `Send`/`Sync` dla danych przechodzących między wątkami - zbadane, werdykt: NIE wdrażać jako proponowane

Rust ma `Send` (bezpiecznie przekazać własność wartości do innego wątku) i `Sync` (bezpiecznie
współdzielić `&T` między wątkami bez synchronizacji) jako marker-trait wymuszane przez kompilator.
C++ nie ma automatycznego odpowiednika, ale dałoby się dobudować lekki `static_assert`-owy wrapper
w konkretnych miejscach, gdzie dane faktycznie przekraczają granicę wątku.

**Co sprawdzono konkretnie:**

- `core/os/thread.h` - `Thread::start(Callback p_callback, void *p_user, ...)`,
  `typedef void (*Callback)(void *p_userdata);` - w pełni type-erased `void*`, zero sprawdzenia
  (nawet runtime) co jest przekazywane.
- `core/object/worker_thread_pool.h:245-246` - `add_task(const Callable&, ...)` i
  `add_native_task(void (*)(void*), void *p_userdata, ...)`. Realny call site:
  `modules/noise/noise_texture_2d.cpp:142,216` -
  `add_task(callable_mp(this, &NoiseTexture2D::_thread_function), ...)` - wiąże `this` (zasób
  `RefCounted`) prosto do closure na wątku roboczym, zero sprawdzenia czasu kompilacji. Ten sam
  wzorzec: `rendering_server_default.cpp:280`, `physics_server_*_wrap_mt.cpp:91`,
  `betsy/image_compress_betsy.cpp:275`.
- `core/core_bind.h:478-497` - `Thread.start(callable)` w GDScript - każdy skrypt może przekazać
  dowolny `Object`/zasób do nowego wątku OS. `doc/classes/Thread.xml` ostrzega tylko prozą
  ("The use of synchronization via Mutex or Semaphore is advised..."), nie jest to wymuszone.
- `core/templates/command_queue_mt.h:125,232` - `static_assert` sprawdza tylko **rozmiar**
  komendy (`sizeof(CommandType) <= MAX_COMMAND_SIZE`), potem `memcpy`/placement-new czegokolwiek -
  brak jakiegokolwiek traita na to, czy typ jest bezpieczny do przeniesienia.
- **Dzisiejsze wymuszanie "thread-safe" - szerokie, ale czysto runtime, dwa warianty:** dedykowane
  makra tylko dla `Node` (`scene/main/node.h:937-940` - `ERR_MAIN_THREAD_GUARD`/
  `ERR_READ_THREAD_GUARD`), i ręczne `ERR_FAIL_COND_MSG(!Thread::is_main_thread(), ...)` powtarzane
  osobno w dziesiątkach miejsc (`scene/main/node.cpp` 6+, `scene_tree.cpp`, `file_dialog.cpp` 4x,
  `rendering_server_default.cpp`, oba `navigation_region_*.cpp`).

**Honest werdykt: NIE wdrażać compile-time `Send`/`Sync`-wrappera dla `Thread`/`WorkerThreadPool`.**
Realny ryzyk niemal zawsze przechodzi przez `Callable`/`Variant` (`add_task(callable_mp(this, ...))`,
`Thread.start(callable)` z GDScript) - w momencie przekroczenia granicy wątku informacja o typie z
C++ już nie istnieje (to `Variant`, dynamicznie typowana wartość runtime), więc żaden C++ trait nie
ma czego sprawdzić. Jedyne miejsca, gdzie trait faktycznie by coś przechwycił
(`Thread::start(Callback, void*)`, `add_native_task`) to rzadko używana, już w pełni type-erased
ścieżka natywna - wrapper strzegłby wąskiego, rzadkiego przypadku, a zostawiłby prawdziwe ryzyko
(`Callable`/`Variant`) całkowicie nietknięte.

**Co faktycznie warto zrobić zamiast tego:** rozszerzyć istniejący wzorzec `ERR_MAIN_THREAD_GUARD`/
`ERR_READ_THREAD_GUARD` (dziś tylko w `Node`) na więcej klas, które dziś mają tylko ręczne, osobno
pisane `is_main_thread()` checki w dziesiątkach miejsc - to już jest godotowy, pragmatyczny
odpowiednik tego, co `Send`/`Sync` robi w Rust, tylko w runtime, nie w typie - rozszerzenie istniejącej
dyscypliny, nie nowa równoległa mechanika.

### Konkretna propozycja (po dogłębnym sprawdzeniu) - dwie małe, realne poprawki

**Sprawdzone dokładnie:** `ERR_MAIN_THREAD_GUARD` (`scene/main/node.h:935-940`) sprawdza
`is_current_thread_safe_for_nodes()`, która jest **głęboko specyficzna dla Node** (patrzy na
`current_process_thread_group`/`data.tree` - logikę grup wątków scene tree). **Nie da się jej
sensownie wyciągnąć jako ogólnej wersji dla innych klas** - to nie jest to samo sprawdzenie.

Ale jest **inny, już ogólny** mechanizm obok: `core/os/thread_safe.h` -
`#define _THREAD_SAFE_CLASS_ mutable Mutex _thread_safe_;` / `#define _THREAD_SAFE_METHOD_ MutexLock _thread_safe_method_(_thread_safe_);`
- już używany w 44 plikach (`ProjectSettings`, `Input`...). To realnie lockuje (nie tylko faili),
więc to inny, ale już istniejący i reużywalny wzorzec.

Natomiast **42 miejsca** (nie 4, jak wstępnie szacowano) w `scene/`+`servers/` mają identyczny,
ręcznie pisany kształt: `ERR_FAIL_COND_MSG(!Thread::is_main_thread(), "<własny tekst>")` - i ten
warunek JEST już ogólny (nie zależy od logiki Node), tylko nikt go nie wyciągnął do makra. Realna
propozycja:

```cpp
#define ERR_FAIL_NOT_MAIN_THREAD_MSG(m_msg) \
    ERR_FAIL_COND_MSG(!Thread::is_main_thread(), m_msg)
#define ERR_FAIL_NOT_MAIN_THREAD_V_MSG(m_ret, m_msg) \
    ERR_FAIL_COND_V_MSG(!Thread::is_main_thread(), m_ret, m_msg)
```

Drop-in zamiennik dla wszystkich 42 miejsc (`scene_tree.cpp:1143,1168,1682,1720,1764,1771,1779`,
`rendering_server_default.cpp:448`, `navigation_region_{2d,3d}.cpp`, `node.cpp` 6+), zero zmiany
zachowania, jedno miejsce do zmiany warunku w przyszłości (np. gdyby Godot dodał drugi "bezpieczny"
wątek). Niewiele skraca kod, ale ujednolica komunikat i robi tę kategorię grepowalną.

**Druga, osobna, realna poprawka** - `WorkerThreadPool::add_task()` (`worker_thread_pool.cpp:347-381`)
dziś **wcale nie sprawdza**, do czego jest związany przekazany `Callable` (`task->callable = p_callable;`,
zero inspekcji). `Callable::get_object()` (`callable.h:107`) już istnieje i jest tani. Realny,
tani dodatek (tylko debug build, ostrzeżenie nie hard-fail, bo są legalne przypadki deferred-safe):

```cpp
#ifdef DEBUG_ENABLED
if (Object *obj = p_callable.get_object()) {
    if (Object::cast_to<Node>(obj)) {
        WARN_PRINT(vformat("Submitting a task bound to a Node (%s) to a worker thread...", obj->get_class()));
    }
}
#endif
```

Łapie najczęstszy przypadek przypadkowego błędu (ktoś wrzucił `Node` do worker threada) w momencie
zgłoszenia zadania, zamiast czekać aż błąd objawi się później jako race gdzieś w głębi.

**Ważne zastrzeżenie przed wdrożeniem:** `Thread.set_thread_safety_checks_enabled()`
(`core/core_bind.cpp:1549-1552`) to już istniejący, sankcjonowany "wyłącznik" tej kategorii checków
(per-wątek, nie globalny, przełącza `thread_local current_thread_safe_for_nodes`). Scripterzy już
mają precedens wyłączania tego typu checków, gdy jest niewygodny - warto to mieć w głowie, gdyby
ktoś pytał "czy to nie będzie zbyt restrykcyjne".

**Werdykt: obie małe poprawki warte zrobienia** - mechaniczne, bez zmiany zachowania, zamykają
realny, dziś istniejący brak (`add_task` zero inspekcji) albo porządkują rozrzucony duplikat kodu
(42 miejsca).

## 9. `_LIFETIME_BOUND_` - brakujące adnotacje na `RBMap`/`RBSet::find()`

Godot już ma **własny, częściowy odpowiednik Rustowego borrow-checkera**: `_LIFETIME_BOUND_`
(`core/typedefs.h:135-145`) próbuje `[[clang::lifetimebound]]`, `[[gnu::lifetimebound]]` (GCC),
`[[msvc::lifetimebound]]`, potem generyczny `[[lifetimebound]]`, i cicho nic nie robi gdy kompilator
nie wspiera - już poprawnie zabezpieczone per-kompilator, nie tylko dla Clanga.

**Adopcja:** 107 użyć w 17 plikach - wszystkie "oczywiste" kontenery już to mają: `Vector::ptr()/ptrw()`,
`LocalVector::ptr()`, `HashSet::find()`, `HashMap::getptr()`, `String`/`CharString::ptr()/ptrw()/get_data()`.

**Konkretna dziura:** `core/templates/rb_map.h:627,636` (`RBMap::find()`, obie wersje const/non-const)
i `core/templates/rb_set.h:578,587` (`RBSet::find()`, to samo) zwracają `Element*`/`const Element*`
do własnego wewnętrznego drzewa - **dokładnie ten sam kształt**, co już oznaczone `HashMap::getptr()`/
`HashSet::find()` - ale bez adnotacji.

**Werdykt: warto zrobić.** 4 linijki, zero zmiany zachowania, domyka realną klasę UB (dangling
pointer do tymczasowego obiektu) na buildach Clang/GCC/MSVC wspierających atrybut - to nie nowy
wzorzec, tylko dokończenie już przyjętej konwencji w projekcie.

## 10. Brakujące wirtualne destruktory - sprawdzone, NIC nie znaleziono (już wymuszone przez build)

Sprawdzono 10 klas bazowych poza hierarchią `Object` z metodami wirtualnymi
(`GodotShape2D`/`3D`, `MIDIDriver`, `AudioDriver`, `RenderingContextDriver`, `GodotBroadPhase2D`,
`Logger`...) - **wszystkie mają wirtualny destruktor**. `Logger` jest nawet realnie usuwany przez
pointer do klasy bazowej (`core/io/logger.cpp:262-263`, `CompositeLogger::~CompositeLogger()` robi
`memdelete(loggers[i])` na `Vector<Logger*>`) - bezpiecznie, właśnie dlatego że destruktor jest
wirtualny.

**Powód:** `SConstruct:1010` już włącza `-Wnon-virtual-dtor` (i `-Wctor-dtor-privacy`) na poziomie
`extra` - kompilator już łapie tę klasę buga w całym projekcie.

**Werdykt: nic do zrobienia** - to nie dziura, to check, który Godot już ma włączony.

## 11. Flagi kompilatora, które realnie mają zęby (sprawdzone eksperymentalnie, nie teoretycznie)

Pełna dzisiejsza lista flag z `SConstruct:945-1033` na poziomie `extra`:
- **GCC:** `-Wall -Wextra -Wwrite-strings -Wshadow -Wenum-conversion -Wctor-dtor-privacy
  -Wnon-virtual-dtor -Walloc-zero -Wduplicated-branches -Wduplicated-cond -Wstringop-overflow=4
  -Wplacement-new=1 -Wvirtual-inheritance -Wattribute-alias=2 -Wlogical-op` (te ostatnie trzy już
  włączone - **korekta wcześniejszego przypuszczenia, że ich nie ma**)
- **Clang:** `-Wall -Wextra -Wwrite-strings -Wshadow-field-in-constructor -Wshadow-uncaptured-local
  -Wenum-conversion -Wctor-dtor-privacy -Wnon-virtual-dtor -Wimplicit-fallthrough`

**AKTUALIZACJA - zweryfikowane realnym, pełnym buildem całego silnika** (nie próbką 5 plików):
włączono 10 kandydatów naraz w `SConstruct` (eksperymentalnie, nie w commicie), zbudowano
`target=editor tests=yes warnings=extra` od zera (`-j6`, ~30 min), log **1.46 GB**. Każda liczba
poniżej to policzona z realnego logu, deduplikowana po unikalnej lokalizacji `plik:linia` (surowe
liczby ostrzeżeń są zawyżone przez powtórki per-instancjacja-szablonu - np. `-Wsuggest-override`
dało 10 318 surowych trafień, ale tylko **385 unikalnych lokalizacji**).

| Flaga | Surowe trafienia | Unikalne lokalizacje | Unikalne poza `thirdparty/` |
|---|---|---|---|
| `-Wold-style-cast` | 1 327 089 | 1443 | 1154 |
| `-Wuseless-cast` | 423 976 | 38 | 29 |
| `-Wsign-conversion` | 288 150 | 481 | 426 |
| `-Wcast-qual` | 153 828 | 119 | 105 |
| `-Wconversion` | 94 465 | 501 | 338 |
| `-Wfloat-conversion` | 63 805 | 558 | 544 |
| `-Wdouble-promotion` | 34 789 | 267 | 251 |
| `-Wsuggest-override` | 10 318 | 385 | 385 |
| `-Wredundant-decls` | 284 | - | ~2 (reszta `thirdparty/mbedtls`) |
| `-Woverloaded-virtual` | 12 | 2 | 2 |

**`-Wsuggest-override` - korekta wcześniejszej (próbkowej) oceny.** Pierwsza próbka (5 plików)
sugerowała "9-liniowy fix w jednym headerze". Pełny build pokazuje inny obraz: **385 unikalnych
miejsc**, z czego `core/object/callable_mp.h` to tylko 7 (powtórzone 7893 razy przez
instancjacje szablonu per-TU) - **reszta to prawdziwe, osobne braki `override`**, skoncentrowane w
backendach renderera: `servers/rendering/renderer_scene_cull.h` (454 wystąpień/wiele unikalnych
metod), `drivers/gles3/storage/material_storage.h` (400), `servers/rendering/renderer_rd/storage_rd/material_storage.h` (280),
`.../environment/sky.h` (160), `.../storage_rd/particles_storage.h` (112), `.../environment/fog.h` (112),
`drivers/gles3/rasterizer_gles3.h` (110), `scene/3d/physics/physical_bone_3d.h` (100),
`.../renderer_canvas_render_rd.h` (80), `core/debugger/remote_debugger.h` (32),
`editor/docks/filesystem_dock.h` (23). Wniosek: dyscyplina `override` w kodzie wysokopoziomowym
(`scene/`) jest czysta, ale backendy renderera (`servers/rendering/renderer_rd/`,
`drivers/gles3/`) systematycznie nigdy nie przyjęły tej konwencji - to jest dużo większy fix niż
myślałem (kilkaset miejsc w ~14 plikach), nie 9 linii.

**`-Woverloaded-virtual` - MAŁY, ale potwierdzony, prawdziwy hit (dokładnie ten typ buga, o który
chodzi):**
- `core/os/os.h:328` - `virtual String OS::get_user_data_dir() const` **was hidden** - realna
  klasa core, metoda w klasie pochodnej chowa (nie nadpisuje) bazową przez niezgodność sygnatury.
- `editor/scene/2d/abstract_polygon_2d_editor.h:142` - `AbstractPolygon2DEditor::_action_set_polygon(int, const Variant&)` **was hidden**.

Tylko 2 unikalne lokalizacje w całym projekcie - **czysty sygnał, zero szumu**, i to jest
dokładnie klasa buga, którą ta flaga ma łapać (coś, co wygląda jak override, ale nie jest).

**`-Wredundant-decls`** - 284 trafień, ale **prawie wszystko to `thirdparty/mbedtls`**
(redeklaracje w PSA crypto API tej biblioteki, nie Godota). Jedyny realny hit w kodzie Godota:
`editor/inspector/editor_inspector.h:146` - redundantne `friend class EditorInspector;` w
`EditorProperty`. Niewiele warte ścigania jako flaga projektowa (szum z thirdparty dominuje), ale
ten jeden one-off można poprawić przy okazji.

**`-Wuseless-cast`** - 29 unikalnych miejsc poza thirdparty, ale **to nie bugi, to stylistyka** -
głównie rzutowania wskaźników do metod w makrach bindingowych (`core/object/object.h:172-317`,
`ClassDB`/`MethodBind` idiom) i nadmiarowe rzutowania liczbowe (`core/math/math_funcs.h`,
`core/math/math_funcs_binary.h`). Małe, ale niska wartość - nic tu nie jest realnym błędem.

**`-Wconversion`/`-Wsign-conversion`/`-Wfloat-conversion`/`-Wdouble-promotion`/`-Wcast-qual`/
`-Wold-style-cast` - potwierdzone empirycznie jako NIE nadające się do włączenia.** Nawet po
deduplikacji (nie surowe miliony, tylko unikalne lokalizacje) to wciąż **105-1154 miejsc każda** w
samym kodzie Godota (nie thirdparty) - zbyt duże, żeby to był "czysty zysk" w stylu
`-Wsuggest-override`/`-Woverloaded-virtual`. Governance tego typu konwersji w Godocie (int/size_t,
float/double, real_t) jest zbyt rozlana po całym kodzie, żeby włączyć to jedną flagą - wymagałoby
osobnej, wieloetapowej kampanii (albo project-wide, albo wcale), nie jest to kandydat na "małą
poprawkę".

**Werdykt (zrewidowany na podstawie realnych liczb):**
- **Włączyć od razu, zero ryzyka:** `-Woverloaded-virtual` (2 realne hity, zero szumu) i
  `-Wimplicit-fallthrough` na GCC (kod już zgodny, 52x `[[fallthrough]]` już w kodzie).
- **Włączyć, ale to większy fix niż sądziłem:** `-Wsuggest-override` - wymaga najpierw dodania
  `override` w ~378 miejscach w backendach renderera (nie 9 linii), ale wszystkie to mechaniczne,
  bezpieczne dodanie jednego słowa kluczowego - możliwe do zrobienia per-plik, nie na raz.
- **Nie ścigać jako projektowa flaga:** `-Wconversion` i cała rodzina (`-Wsign-conversion`,
  `-Wfloat-conversion`, `-Wdouble-promotion`, `-Wcast-qual`, `-Wold-style-cast`) - potwierdzone
  empirycznie jako zbyt szerokie (setki-tysiące unikalnych miejsc) na jednorazową kampanię.
- **Opcjonalnie, bardzo niski priorytet:** `-Wuseless-cast` (29 miejsc, ale stylistyka, nie bugi),
  `-Wredundant-decls` (1 realny hit w Godocie, reszta to szum z `thirdparty/mbedtls`).

## 12. `NonCopyable` - konstrukt jak `OwnedMutex`, ale dla "nie kopiuj mnie" (double-free)

Rust nie ma w ogóle tej klasy buga - typ jest move-only, jeśli nie zaimplementuje jawnie `Clone`.
C++ wymaga explicit `=delete` na konstruktorze kopiującym; łatwo o tym zapomnieć w klasie z
customowym destruktorem, który coś zwalnia/zamyka - domyślny, skompilowany-za-darmo konstruktor
kopiujący zrobi shallow copy, i dwa obiekty będą próbowały zwolnić **ten sam** zasób.

**Nie istnieje żaden współdzielony mixin/makro `NonCopyable`** - tylko rozrzucone, pojedyncze
`=delete` (`core/templates/pair.h:68-69`, `core/templates/self_list.h:201,233`,
`core/variant/variant.h:412-413`, `core/templates/owned_mutex.h:53` - ten dodany w tej sesji).
`GDCLASS` (`core/object/object.h:248`) usuwa tylko `operator=`, nie konstruktor kopiujący.

**Ważne odkrycie: większość klas jest dziś chroniona przypadkiem.** `Thread`
(`core/os/thread.h:106`, pole `std::thread`) i `MutexLock` (`core/os/mutex.h:71`, pole
`std::unique_lock`) mają pole z biblioteki standardowej, które samo jest niekopiowalne - kompilator
automatycznie usuwa im konstruktor kopiujący, bez jawnego `=delete`. Ochrona jest prawdziwa, ale
przypadkowa, nie zaprojektowana.

**Realny, niechroniony przypadek (2 klasy):**

- **`RendererSceneCull::Instance`** (`servers/rendering/renderer_scene_cull.h:613-615`):
  ```cpp
  ~Instance() {
      memdelete(base_data);
      memdelete(custom_aabb);
  }
  ```
  `base_data`/`custom_aabb` to gołe pointery, reszta pól to skalary/RID - nic nie chroni tej klasy
  przypadkiem. Domyślny konstruktor kopiujący zrobiłby shallow copy obu pointerów → dwa obiekty,
  dwa destruktory, `memdelete` na tym samym pointerze dwa razy. Sprawdzono realne call site'y - nie
  znaleziono żadnej kopii przez wartość (wszędzie `Instance*`). **Teoretyczne dziś, nie
  wyzwolone, ale zero-cost do zamknięcia** (`Instance(const Instance&) = delete;` + `operator=`).

- **`PipelineDeferredRD`** (`servers/rendering/renderer_rd/pipeline_deferred_rd.h:63-87`) - posiada
  `RID pipeline` i `WorkerThreadPool::TaskID task`, cleanup odłożony do `free()` (nie destruktora -
  destruktor tylko `ERR_FAIL_COND_MSG` w `DEV_ENABLED`, jeśli `free()` nie było wywołane - to
  dodatkowo ten sam typestate problem co sekcja 7). Kopia tej klasy dałaby dwa obiekty z tym samym
  `RID`/`TaskID`, bez współdzielonego śledzenia "już zwolnione".

**Zasięg - sprawdzono tylko 5 klas, 2/5 to realne dziury** (`Thread`, `MutexLock`,
`AudioDriverDummy` są chronione przypadkiem przez pole `Thread`/`Mutex`). Pełny sweep (szukać
"customowy destruktor + zero pola `Mutex`/`Thread`/`unique_lock`/socket") nie został zrobiony.

**Werdykt: warto zrobić wąsko (2 znalezione klasy), nie jako kampania.** Współdzielony
`NonCopyable` mixin w `core/typedefs.h` byłby łatwy do napisania i rozsądny do dodawania
opportunistycznie przy dotykaniu plików, ale przy 2 potwierdzonych kandydatach (bez udokumentowanego
realnego incydentu) to "tanie zabezpieczenie", nie pilny refaktor.

## 13. `BitField<T>` - już istnieje, dobrze przyjęty, dwa konkretne, tanie braki

Godot ma już type-safe wrapper na flagi bitowe: `BitField<T>` (`core/templates/bit_field.h:40`) -
`static_assert(std::is_enum_v<T>)`, przechowuje `uint64_t`, ale jest sparametryzowany konkretnym
enumem `T`. `BitField<EnumA>` i `BitField<EnumB>` to **różne typy** - kompilator odrzuci pomieszanie
flag z dwóch różnych enumów na granicy funkcji (to jest ten sam duch co `OwnedMutex`/`RID` - typ,
nie konwencja, pilnuje poprawności). Adopcja: **624 użycia w 154 plikach**, głównie jako
parametr/zwrot w publicznych, bindowanych do scriptingu metodach (`core/io/file_access*.h`,
`core/core_bind.h`) - ta część wzorca jest już dojrzała.

**Dwa konkretne, tanie kandydaty na migrację** (pole przechowujące flagi jest wciąż gołym `int`,
mimo że odpowiadający enum już istnieje):

1. **`Skeleton3D::update_flags`** (`scene/3d/skeleton_3d.h`, `uint8_t update_flags = UPDATE_FLAG_NONE;`)
   budowane z prywatnego `enum UpdateFlag { UPDATE_FLAG_NONE=1, UPDATE_FLAG_MODIFIER=2, UPDATE_FLAG_POSE=4 }`
   (linie 91-93), składane przez `|=`/`&` w `skeleton_3d.cpp` (linie 341,376,382,384,461,1007,1014,1019,1022).
   Czysto wewnętrzne, nie eksponowane do scriptingu - najniższe ryzyko z dwóch.

2. **`BaseMaterial3D::stencil_flags`** (`scene/resources/material.h:623`, `int stencil_flags = 0;`)
   budowane z `enum StencilFlags { STENCIL_FLAG_READ=1, WRITE=2, WRITE_DEPTH_FAIL=4 }` (linie 339-344),
   który **już jest** `VARIANT_ENUM_CAST`'owany (linia 910) - czyli już widoczny w scriptingu jako
   enum flag - a `set_stencil_flags(int)`/`get_stencil_flags() const` (linie 847-848) wciąż biorą/
   zwracają goły `int`. To jest publiczne API, więc migracja zmienia sygnaturę bindingu, ale 624
   istniejących użyć `BitField<T>` jako bindowanego parametru/zwrotu już dowodzi, że to działa
   bez problemu - nie jest to nowe ryzyko.

**Werdykt: warto zrobić, tanie.** Oba to małe, zlokalizowane zmiany jednego pola (nie kampania) -
enum już istnieje, typ już istnieje, zmienia się tylko deklarowany typ pola przechowującego flagi.

## 14. Printf-format attribute - sprawdzone, ZAMKNIĘTE (Godot już to robi dobrze)

Sprawdzono, czy funkcje formatujące Godota mają `__attribute__((format(printf,...)))` (pozwala
kompilatorowi automatycznie sprawdzać zgodność format-stringa z argumentami na każdym call site).

`vformat()` (`core/variant/variant.h:922`) to C++ template wariadyczny (`VarArgs... p_args`), nie
C-style `...` - atrybut `format(printf,...)` **nie da się go w ogóle zastosować** (działa tylko na
prawdziwych `va_list`-funkcjach). To jest realna, ale strukturalnie inna dziura, której ten atrybut
nie zamknie.

Prawdziwe C-wariadyczne funkcje printf-style - `OS::print/print_rich/printerr`
(`core/os/os.h:170-172`), `Logger::logf/logf_error` (`core/io/logger.h:90-91`) - **wszystkie już
mają** własne makro Godota `_PRINTF_FORMAT_ATTRIBUTE_2_3` (`core/typedefs.h:237-241`).

**Werdykt: nic do zrobienia** - Godot już zamknął tę dziurę wszędzie, gdzie atrybut faktycznie da
się zastosować.

## 15. `-Wempty-body`/`-Wnull-dereference` - wstępny sygnał dobry, niepotwierdzony pełnym buildem

Szybki test (`-fsyntax-only`, bez pełnego rebuildu silnika - żeby nie zajmować CPU) na **45
losowych plikach** z `core/`/`scene`/`servers/`: **zero trafień** dla obu flag. To nie jest pełny
audyt (jak sekcja 11), tylko próbka - sygnał jest dobry, ale niepotwierdzony na całym projekcie.

**Werdykt: prawdopodobnie warto, ale wymaga pełnego buildu do ostatecznego potwierdzenia** (jak
`-Wsuggest-override`, gdzie próbka 5 plików dała zupełnie inny obraz niż pełny projekt) - nie
polegać na tej próbce jako ostatecznej ocenie.

## 16. Funkcje z 2+ kolejnymi `bool` - łatwe do pomylenia na call site

Funkcja `foo(bool p_enable, bool p_recursive)` pozwala wywołać `foo(true, false)` - nic nie
odróżnia kolejności, kompilator nie widzi różnicy między dwoma `bool`. Standardowy fix: zamiana
pozycyjnych boolów na nazwany `enum class` (`enum class Sync { No, Yes };`) - wtedy pomylenie
kolejności dwóch różnie nazwanych enumów jest błędem kompilacji (złe typy), nie cichym bugiem.

**`AnimationTree::blend_node`/`blend_input`** (`scene/animation/animation_tree.h:177-178`) -
`blend_node(..., FilterAction p_filter = FILTER_IGNORE, bool p_sync = true, bool p_test_only = false)`.
Realne, **niekonsekwentne** call site'y: `scene/animation/animation_blend_space_2d.cpp:725,781` i
`animation_blend_space_1d.cpp:534,590` piszą `blend_node(..., FILTER_IGNORE, true, true)` - dwa
gołe `true` pod rząd, zero wskazówki który to `p_sync` a który `p_test_only` - podczas gdy inne
wywołania w tych samych plikach (linie 785,798,452,594,607) piszą `..., true, p_test_only)`. Ta
niekonsekwencja **dowodzi**, że te dwa parametry są realnie w ruchu i mylone, nie tylko teoretycznie
mylne. Publiczne API (`doc/classes/AnimationNode.xml`) ma ten sam kształt na poziomie scriptingu.

Niżej priorytetowo: `CharacterBody2D::_snap_on_floor` (`scene/2d/physics/character_body_2d.h:160`,
3 bole pod rząd, wywołanie `character_body_2d.cpp:200`) - metoda wewnętrzna (`_`-prefiks), mniej
istotna niż publiczne API.

**Werdykt: warto, wąsko.** `blend_node`/`blend_input` to najlepszy kandydat - publiczne API,
realna niekonsekwencja w wywołaniach już dziś, nie hipoteza.

## 17. Sentinel-value + brak `ScopeGuard`/defer - jeden pusty trop, jeden mocny

**Sentinel-value (`-1`/`nullptr` jako "nie znaleziono") - sprawdzono ~10 realnych
`.find()`→index miejsc w `scene/`/`editor/`/`core/` - wszystkie poprawnie sprawdzają `-1` przed
użyciem.** Brak żywego, niesprawdzonego przypadku w tej sesji. Ale historia tego repo pokazuje,
że ten typ buga **już się zdarzył**: commit `a23610ad90` "Fix crash when using invalid index in
Color.get_named_color" (2021) - `Color::get_named_color(int p_idx)` nie miało żadnej walidacji
indeksu, naprawione dodaniem `ERR_FAIL_INDEX_V`. **Werdykt: brak żywego dowodu teraz, ale
potwierdzona historia tej klasy buga w repo - niski priorytet bez dalszego researchu, nie
odrzucać całkowicie.**

**`ScopeGuard`/defer - Godot NIE ma ogólnego narzędzia tego typu**, ale znaleziono mocny,
konkretny przypadek, który dowodzi, że jest potrzebny. Jedyny `class Defer` w kodzie
(`core/profiling/profiling.h:193`) jest prywatny dla `namespace apple::instruments`, kompilowany
tylko pod `GODOT_USE_INSTRUMENTS`, bierze goły wskaźnik funkcji bez capture - nie jest to
reużywalne narzędzie ogólnego przeznaczenia.

**`WorkerThreadPool::wait_for_task_completion()`** (`core/object/worker_thread_pool.cpp:403-465`)
ręcznie woła `task_mutex.lock()`/`.unlock()` **5 razy** w jednej funkcji, z 3 wczesnymi
`return`/`ERR_FAIL_V_MSG` (linie 408,416,433) - każdy musi pamiętać `.unlock()` przed wyjściem.
Zero siatki bezpieczeństwa - przyszła edycja dodająca 4. early-return i zapominająca `.unlock()`
to cichy deadlock, niewykrywalny do produkcji. Ten sam wzorzec w `_process_task()` w tym samym
pliku (linie 77,85,138,150,178).

**Najciekawsze: Godot już ma narzędzie, które by to naprawiło bez pisania nowego kodu** -
`MutexLock` (`core/os/mutex.h:71-100`) ma metody `temp_relock()`/`temp_unlock()` zaprojektowane
dokładnie pod ten wzorzec (odblokuj w środku zasięgu, zablokuj później, destruktor i tak
gwarantuje finalny unlock przy każdym wyjściu) - ale `wait_for_task_completion()`/`_process_task()`
go nie używają, robią wszystko ręcznie.

**Werdykt: warto, i to bez pisania nowego kodu** - przepisanie `wait_for_task_completion`/
`_process_task` na istniejący `MutexLock::temp_relock()`/`temp_unlock()` zamyka realne ryzyko
deadlocku przy przyszłej edycji, używając narzędzia, które Godot już ma, tylko nie tutaj.

## 18. Walidacja na realnych issues z GitHuba + jedno duże nowe znalezisko (Clang Thread Safety Analysis)

Pobrano i przefiltrowano issues z `godotengine/godot` (zapytania: "race condition", "deadlock",
"thread safety", "data race", "not thread safe" - closed, żeby widzieć realny fix). Wynik: **to nie
jest teoria** - prawie każdy wzorzec buga znaleziony w tej sesji przez czytanie kodu **już się
realnie zdarzył i był zgłoszony jako crash w produkcji**.

**Bezpośrednie potwierdzenia wcześniejszych znalezisk:**

- **#118450** "AudioServer::add_bus() and move_bus() modify buses vector without audio driver
  lock" - **dokładnie** ten sam plik i mechanizm, co niekonsekwencja lockowania w `AudioServer`
  znaleziona wcześniej w tej sesji (wtedy znalazłem `set_bus_volume_db`/`set_bus_send`/etc. bez
  locka; to zgłoszenie znalazło `add_bus()`/`move_bus()` - inne konkretne metody, ten sam
  systemowy problem). `remove_bus()`/`set_bus_name()` poprawnie lockują, `add_bus()`/`move_bus()`
  nie - ten sam "locked-niektóre/unlocked-inne" wzorzec jak `ResourceUID`/`CallQueue`.
- **#114896** "Race condition in `MaterialStorage::material_set_shader`" (`servers/rendering/renderer_rd/storage_rd/material_storage.cpp`) -
  crash w produkcji (`HashSet::insert`/`erase` access fault) przy ładowaniu wielu meshy na
  wątkach roboczych odwołujących się do tego samego shadera. Potwierdzony, zamknięty, realny bug -
  ten sam "brak locka na mutacji współdzielonej struktury" bug class.
- **#113210** "`Variant::get_type_by_name` `HashMap` initialization is not thread-safe" -
  **podręcznikowy double-checked lazy-init race**: statyczny `HashMap` sprawdzany "czy pusty,
  jeśli tak - wypełnij" bez synchronizacji; dwa wątki widzą pustą mapę naraz i oba próbują wypełnić
  → `double free or corruption`. **To jest dokładnie ten bug, który C++11 "magic statics"
  (funkcja-lokalny `static` z gwarantowaną przez standard thread-safe inicjalizacją) eliminuje
  za darmo, bez żadnego ręcznego locka** - gdyby to było napisane jako
  `static HashMap<...> &get_map() { static HashMap<...> map = build_map(); return map; }`,
  kompilator/runtime same zagwarantowałyby, że inicjalizacja zdarzy się raz, bezpiecznie, nawet
  pod wyścigiem wątków - zero ręcznego kodu synchronizacji. To jest nowy, bardzo tani wzorzec do
  rekomendacji: **wszędzie, gdzie jest statyczny/lazy-init kontener sprawdzany "czy pusty" przed
  wypełnieniem, zamienić na function-local static** - eliminuje całą klasę buga strukturalnie.
- **#63461** "Race condition / use-after-free in `ResourceLoader::load_threaded_get`" - ten sam
  moduł (resource loading), ta sama klasa buga, co fix `ResourceUID` z tej sesji - potwierdza, że
  to jest systemowo powtarzający się problem w tej części silnika, nie jednorazowy przypadek.
- **[#50470](https://github.com/godotengine/godot/issues/50470)** "Area2D can crash with
  multi-threaded physics" - **wciąż OTWARTE, zweryfikowane w aktualnym kodzie (nie tylko z opisu
  z 2021)**: `GodotArea2D::set_transform`/`_shapes_changed`/`set_space`/`set_monitor_callback`/
  `set_area_monitor_callback` (`modules/godot_physics_2d/godot_area_2d.cpp`, 5 miejsc) robią
  `if (!moved_list.in_list() && get_space()) { get_space()->area_add_to_moved_list(&moved_list); }` -
  check-then-act na gołym intrusive `SelfList<T>` (`core/templates/self_list.h`, zero wewnętrznej
  synchronizacji). `GodotSpace2D` ma **zero mutexów** w całej klasie. Ten sam bug class co
  `AudioServer`/`MaterialStorage`, tylko w fizyce 2D/3D - rozszerza zasięg sekcji 1/11 o kolejną
  domenę silnika, nie wprowadza nowej kategorii problemu. 4+ lata otwarte, wciąż niezreplikowane
  w stałą naprawę - konkretny dowód kosztu braku `-Wthread-safety`/systemowego `OwnedMutex`.

**Duże, nowe znalezisko: Clang Thread Safety Analysis (`-Wthread-safety`) - całkowicie
nieużywane w Godocie.** Sprawdzono (`grep -rn "GUARDED_BY\|ACQUIRED_BEFORE\|LOCKABLE" core/ servers/ scene/`)
- **zero wystąpień**. To jest wbudowany w Clang mechanizm **statycznej analizy w czasie
  kompilacji**, który pozwala oznaczyć mutex jako `LOCKABLE`, pole jako `GUARDED_BY(mutex)`, i
  metodę jako `EXCLUSIVE_LOCKS_REQUIRED(mutex)` - kompilator **odmawia skompilowania** dostępu do
  oznaczonego pola bez trzymania odpowiedniego locka, i **wykrywa niekonsystentny porządek
  blokowania** dwóch mutexów (`ACQUIRED_BEFORE`/`ACQUIRED_AFTER`) na etapie kompilacji, zanim
  dojdzie do deadlocku w produkcji.

To jest **dokładnie** mechanizm, który złapałby **każdy** z powyższych bugów (#118450, #114896,
#113210 poniekąd, oraz `ResourceUID`/`CallQueue`/`RemoteDebuggerPeerTCP` znalezione wcześniej w
tej sesji) na etapie kompilacji, zamiast czekać aż użytkownik zgłosi crash. To jest mocniejsza,
bardziej ogólna wersja tego, co `OwnedMutex` robi punktowo dla jednej klasy - `-Wthread-safety`
robi to dla **każdego** pola oznaczonego `GUARDED_BY` w całym projekcie naraz.

**Zastrzeżenia (żeby być honest, nie sprzedawać to jako magiczne rozwiązanie):**
- Tylko Clang (nie GCC - Godot domyślnie buduje z GCC na Linuksie, widziane w tej sesji), ale
  Godot już buduje z Clangiem na części platform (macOS/iOS/Web) - działałoby tam od razu, na GCC
  nie.
- Wymaga **jawnego oznaczenia** każdego mutexa i każdego strzeżonego pola - to nie jest flaga
  włączana jednym przełącznikiem jak `-Wsuggest-override`, to adnotacje w kodzie (podobnie jak
  `_LIFETIME_BOUND_` już w projekcie) - realna, ale rozłożona w czasie praca, nie jednorazowa
  zmiana.
- `core/os/mutex.h`'s `Mutex`/`MutexLock` trzeba by oznaczyć (`LOCKABLE`/`SCOPED_LOCKABLE`) jako
  pierwszy krok - to jest **jedna zmiana w jednym pliku**, która odblokowuje użycie adnotacji
  wszędzie indziej (podobny kształt do `OwnedMutex`/`_LIFETIME_BOUND_` - mały fundament,
  rozszerzany opportunistycznie).

**Inne klasy bugów z realnych issues, których NIC z tej sesji nie rozwiązuje (uczciwe
ograniczenia):**
- **#103879** "Deadlock between `PipelineHashMapRD::local_mutex` and `GDScriptCache::mutex`" -
  **deadlock przez niekonsystentny porządek blokowania dwóch różnych mutexów** z dwóch różnych
  ścieżek kodu. `OwnedMutex` to NIE rozwiązuje (zapobiega "zapomnianemu lockowi", nie "zły porządek
  dwóch locków") - to jest dokładnie przypadek, gdzie `-Wthread-safety`'s `ACQUIRED_BEFORE`/
  `ACQUIRED_AFTER` by pomogło, reszta rozwiązań z tej sesji nie.
- **#120941** "`CommandQueueMT`: deadlock when contending with an in-progress flush" - **lost
  wakeup** (wyścig między flush() a sprawdzeniem flaga/warunek) - to jest błąd logiki protokołu
  synchronizacji (zmienna warunkowa/flaga), nie błąd typu - żaden wrapper type tego nie złapie,
  wymaga poprawnego przemyślenia protokołu.
- Potwierdzona, zamknięta w 2024 **masowa fala realnych data race'ów wykrytych przez
  ThreadSanitizer** (#87994 `NetSocketPosix::close()`, #87995 `EditorExportPlatformWeb` destruktor,
  #87997 `AudioServer::update()`, #87998 `AudioDriver::reset_profiling_time()`, #88079
  `EditorFileSystem::_reimport_thread`, #76581 `ScriptServer::register_language`/`thread_enter`) -
  dowód, że TSan **już raz** znalazł realne bugi masowo w tym projekcie, ale nie jest to część
  regularnego CI (inaczej te zgłoszenia by nie były potrzebne) - **uruchamianie TSan okresowo
  (nie tylko ad-hoc) to osobna, wartościowa rekomendacja**, niezależna od wszystkiego innego w
  tym pliku.

**Werdykt: to jest najsilniejszy, najbardziej ugruntowany wniosek z całej sesji.** Rzeczy
znalezione przez czytanie kodu (sekcja 1) nie są teoretyczne - ten sam bug class crashuje
użytkowników w produkcji regularnie. `-Wthread-safety` jest nowym, dużym, nieodkrytym wcześniej
kandydatem, który adresuje to systemowo (nie punktowo jak `OwnedMutex`), ale wymaga realnej,
rozłożonej w czasie pracy adnotacyjnej, nie jednej flagi.

## 19. Znana, trudna granica: reentrant mutex + cooperative work-stealing thread pool

**Issue [#122539](https://github.com/godotengine/godot/issues/122539)** (wciąż OTWARTE, nie
naprawione) - crash w `GDScriptAnalyzer::resolve_class_inheritance` (`CowData<char32_t>::size()`,
prawdopodobnie uwolniony bufor `String`) przy `ResourceLoader.load_threaded_request()` z wieloma
współzależnymi skryptami GDScript. To jest **inny, głębszy rodzaj problemu** niż wszystko inne w
tym pliku - żadne z proponowanych rozwiązań (`OwnedMutex`, `-Wthread-safety`) go nie łapie.

**Mechanizm (zweryfikowany w realnym kodzie, nie tylko z opisu issue):**

`GDScriptCache::mutex` to `SafeBinaryMutex` (`core/os/safe_binary_mutex.h`) - **celowo
reentrant na tym samym wątku** (licznik w `thread_local`), bo rozwiązywanie zależności między
skryptami jest z natury rekurencyjne (`GDScriptParserRef::raise_status()` →
`GDScriptCache::get_parser()` → rekurencyjnie kolejne `raise_status()` dla zależności).

Jednocześnie `WorkerThreadPool::_wait_collaboratively()` (`core/object/worker_thread_pool.cpp:568`)
implementuje **cooperative work-stealing**: wątek, który "czeka" na zadanie, nie blokuje się -
podkrada i wykonuje **inne, niezwiązane zadanie z kolejki** (`_process_task(task_to_process)`) na
tym samym stosie wywołań.

Połączenie tych dwóch mechanizmów: reentrant mutex widzi "ten sam wątek" i traktuje to jako
bezpieczną rekurencję **tej samej logicznej operacji** - ale przez work-stealing to mogą być
**dwa zupełnie niezależne zadania** (dwa różne skrypty), przeplatające się na jednym OS-thread,
mutujące te same współdzielone mapy (`parser_map`, `dependencies`) bez prawdziwej separacji.

**Maintainerzy Godota już to zauważyli - ale naprawili tylko jedno miejsce.** W
`GDScriptCache::get_full_script()` (`modules/gdscript/gdscript_cache.cpp:384-390`):
```cpp
// Allowing lifting the lock might cause a script to be reloaded multiple times,
// which, as a last resort deadlock prevention strategy, is a good tradeoff.
uint32_t allowance_id = WorkerThreadPool::thread_enter_unlock_allowance_zone(singleton->mutex);
r_error = script->reload(true);
WorkerThreadPool::thread_exit_unlock_allowance_zone(allowance_id);
```
To jest **jawnie przyznany trade-off** - świadomie akceptują ryzyko "skrypt przeładowany kilka
razy" w zamian za unikniecie deadlocku. Ale ścieżka crashująca w #122539
(`raise_status()`/`get_parser()`'s własna rekurencja) **nie rejestruje się jako "unlockable
zone"** - nie ma tej samej, świadomej ochrony.

**Czemu żadne z wcześniejszych rozwiązań tego nie łapie:** z perspektywy typu/locka, mutex JEST
trzymany (przez "ten wątek") - `-Wthread-safety`'s `GUARDED_BY`/`EXCLUSIVE_LOCKS_REQUIRED` by to
zaakceptowało jako poprawne, bo dla niego "ten sam thread ID = bezpieczny dostęp". `OwnedMutex`
też nie pomoże - problem nie jest "ktoś zapomniał zablokować", lock jest trzymany cały czas.
Prawdziwy problem jest konceptualny: **mutual exclusion oparty na identyczności wątku łamie się,
gdy jeden OS-thread wykonuje zagnieżdżone, logicznie niezależne zadania** przez cooperative
work-stealing - potrzeba czegoś w stylu task-local (nie thread-local) tożsamości "kto trzyma
lock", co żaden z konstruktów w tym pliku nie adresuje.

**Werdykt: udokumentować jako znaną, świadomą granicę, nie próbować naprawić prostym
konstruktem.** To wymagałoby albo (a) rozszerzenia `thread_enter_unlock_allowance_zone` na
ścieżkę `raise_status()`/`get_parser()` (ten sam trade-off, który maintainerzy już zaakceptowali
gdzie indziej - najmniejszy krok, ale wciąż akceptuje "czasem przeładuje dwa razy" jako cenę za
brak deadlocku), albo (b) fundamentalnego przeprojektowania identyfikacji "kto trzyma lock" z
thread-local na task-local w całym `WorkerThreadPool`/`SafeBinaryMutex` - zbyt duża zmiana na
"prostą poprawkę".

## 20. Systemowy double-checked lazy-init race na statycznych kontenerach - potwierdzone w 2+ miejscach, zgłoszone w 3 kolejnych

Druga runda wyszukiwania issues (filtr użytkownika: `is:issue state:open crash thread
-label:topic:dotnet -label:platform:android -label:discussion`) znalazła **bezpośrednią
kontynuację** znaleziska z sekcji 18 (`Variant::get_type_by_name`, #113210) - inny użytkownik
poszedł szukać więcej takich przypadków i znalazł kolejne.

**[#113865](https://github.com/godotengine/godot/issues/113865)** "Non-thread-safe static
`HashMap` initializations" (OPEN) - lista 4 dodatkowych podejrzanych miejsc z tym samym wzorcem
("czy pusty → wypełnij" bez locka):
- `Color::find_named_color` - **zweryfikowane w aktualnym kodzie, wciąż żywe**:
  ```cpp
  int Color::find_named_color(const String &p_name) {
      ...
      static HashMap<String, int> named_colors_hashmap;
      if (unlikely(named_colors_hashmap.is_empty())) {   // check bez locka
          ...wypełnianie mapy...                          // act bez locka
      }
  ```
  (`core/math/color.cpp:412-420`). Reproduktor z 10 wątków w issue odtwarza crash ~1/20 razy.
- `VariantParser::parse_value` (cache dla typed arrays/dictionaries) - zgłoszone, niezweryfikowane
  w tej sesji.
- `EngineUpdateLabel::_get_version_type` - zgłoszone, niezweryfikowane.
- `_find_annotation_arguments` (`modules/gdscript/gdscript_editor.cpp`) - zgłoszone, niezweryfikowane.

**To silnie wzmacnia rekomendację z sekcji 18**: to nie jest jeden odosobniony przypadek
(`Variant::get_type_by_name`), to **powtarzający się idiom w całym silniku**, z co najmniej 2
potwierdzonymi i 3 dalszymi zgłoszonymi instancjami. Fix jest wszędzie identyczny i tani: zamiana
na function-local `static` z C++11-gwarantowaną thread-safe inicjalizacją (zero ręcznego kodu
synchronizacji).

**[#124010](https://github.com/godotengine/godot/issues/124010)** "GDScript: race in
`OPCODE_OPERATOR` first-run cache crashes when a function is first called from several threads"
(OPEN, zgłoszone jako wciąż żywe na dzisiejszym masterze) - **pokrewny, ale inny mechanizm,
zweryfikowany w aktualnym kodzie** (`modules/gdscript/gdscript_vm.cpp:779-810`):
```cpp
if (unlikely(op_signature == 0)) {
    static Mutex initializer_mutex;      // sam w sobie OK (function-local static)
    initializer_mutex.lock();
    ...
    if (_code_ptr[ip + 5] == 0) {
        _code_ptr[ip + 5] = actual_signature;   // (1) "flaga gotowości" publikowana PIERWSZA
        _code_ptr[ip + 6] = static_cast<int>(ret_type);
        *tmp = op_func;                          // (2) wskaźnik funkcji zapisany PO fladze
    }
    initializer_mutex.unlock();
} else if (likely(op_signature == actual_signature)) {
    // fast path - CZYTA bez locka i bez żadnej bariery pamięci
    Variant::ValidatedOperatorEvaluator op_func = *reinterpret_cast<...>(&_code_ptr[ip + 7]);
    op_func(a, b, dst);   // może wywołać niekompletnie zapisany wskaźnik → crash 0xC0000005
```
To NIE jest "zapomniany lock" - writer **ma** mutex. Problem jest głębszy: (a) fast path czyta
bez locka I bez bariery pamięci, (b) kolejność zapisu jest odwrotna do bezpiecznej (flaga
"gotowości" `ip+5` publikowana ZANIM wskaźnik funkcji `ip+7` jest w pełni zapisany) - klasyczny
"publish flag before payload is complete" race. Tani fix istnieje, ale jest subtelny: flaga
gotowości musi być zapisana z `std::atomic`/release semantics PO zapisaniu payloadu, a fast path
musi czytać flagę z acquire semantics przed odczytem payloadu - samo dodanie locka do fast path
by zabiło całą optymalizację, dla której ten kod istnieje (lock na każdej operacji arytmetycznej
w VM). To jest przypadek, gdzie `OwnedMutex` nie wystarczy - potrzebne `std::atomic` z właściwą
kolejnością pamięci, nie goły mutex.

**Dwa kolejne potwierdzenia tego samego "shared cache, check-then-create" bug class** (ten sam
rodzic co `ResourceUID`):
- **[#98179](https://github.com/godotengine/godot/issues/98179)** "Concurrent threaded
  Instantiating of nodes from same PackedScene causes crash: 'Trying to unreference a
  SafeRefCount'" - dwa wątki instancjonujące scenę z tej samej ścieżki `PackedScene` naraz
  korumpują współdzielony `Ref<Resource>` w cache zasobów, prowadząc do podwójnego unref (crash
  explicite mówiący "no multiple threads should be racing to do the final unreferencing to
  zero").
- **[#34752](https://github.com/godotengine/godot/issues/34752)** "Crash when connecting or
  disconnecting a signal from multiple threads" (22 komentarze) - **uczciwie przyznane przez
  maintainerów jako architektonicznie trudne**, nie fixowalne jednym lockiem: cache'owane zasoby
  nie są unikalne między wątkami, typy jak `AudioStreamPlayer` łączą sygnały bezwarunkowo przy
  tworzeniu z dowolnego wątku. Ten sam rodzaj "znanej, trudnej granicy" co #122539 w sekcji 19 -
  nie każdy race w tym silniku ma prosty fix.

**Werdykt: sekcja 18's rekomendacja (function-local static) jest teraz potwierdzona jako
systemowy problem, nie jednostkowy przypadek** - konkretna, gotowa do zrobienia lista: `Color::find_named_color`
(zweryfikowane) + `Variant::get_type_by_name` (już w sekcji 18) + 3 zgłoszone-do-zweryfikowania.
Opcode cache w GDScript VM (#124010) to **osobny, trudniejszy przypadek** wymagający
`std::atomic`, nie gołego mutexa - warto zaznaczyć jako odrębny fix, nie wrzucać do tego samego
wiadra.
