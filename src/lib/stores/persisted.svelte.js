// A reactive value backed by localStorage. Reads persist across reloads and
// app restarts; reading `.value` is reactive (tracks in $effect/markup),
// writing `.value` updates state and storage. JSON-serialized, so it works for
// strings, numbers, booleans, and plain objects.
//
//   const tint = persisted('tint', 'warm');
//   tint.value          // read (reactive)
//   tint.value = 'blue' // write (also saved)

export function persisted(key, initial) {
  let value = $state(read(key, initial));

  return {
    get value() {
      return value;
    },
    set value(next) {
      value = next;
      try {
        localStorage.setItem(key, JSON.stringify(next));
      } catch {
        // storage unavailable/full — fall back to in-memory only
      }
    },
  };
}

function read(key, fallback) {
  try {
    const raw = localStorage.getItem(key);
    return raw === null ? fallback : JSON.parse(raw);
  } catch {
    return fallback;
  }
}
