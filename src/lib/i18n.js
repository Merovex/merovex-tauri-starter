// Minimal i18n scaffold. Centralize user-facing strings here and read them with
// t('key'). To localize, swap `dict` per locale (or load it dynamically) — the
// call sites don't change. This is a starting point, not full coverage.

const en = {
  'app.untitled': 'Untitled',
  'doc.none': 'No document open.',
  'action.new': 'New',
  'action.open': 'Open',
  'action.delete': 'Delete',
};

const dict = en;

export function t(key, fallback) {
  return dict[key] ?? fallback ?? key;
}
