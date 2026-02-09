// RegExp constructor with unicode property escapes
const IDENTIFIER_RE = new RegExp('\\p{ID_Start}', 'u');
const ASCII_RE = new RegExp('\\p{ASCII}', 'u');

// RegExp called without new
const LETTER_RE = RegExp('\\p{L}', 'u');

// RegExp constructor without unicode property escapes should not be transformed
const PLAIN_RE = new RegExp('[a-z]', 'u');

// RegExp constructor with negated unicode property escape
const NON_ASCII_RE = new RegExp('\\P{ASCII}', 'u');
