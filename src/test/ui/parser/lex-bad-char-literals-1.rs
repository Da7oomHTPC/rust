// compile-flags: -Z parse-only -Z continue-parse-after-error
static c3: char =
    '\x1' //~ ERROR: numeric character escape is too short
;

static s: &'static str =
    "\x1" //~ ERROR: numeric character escape is too short
;

static c: char =
    '\●' //~ ERROR: unknown character escape
;

static s: &'static str =
    "\●" //~ ERROR: unknown character escape
;

