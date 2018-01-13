
## Scheme R7RS definitions

|      Library       |    Category    |        Type        |  Scheme identifier                 |  Rust value
|       :---:        |     :---:      |       :---:        |  :---                              |  :---
|        base        |    syntaxes    |  auxiliary-syntax  | `_                               ` | `Auxiliary`
|        base        |    syntaxes    |  auxiliary-syntax  | `...                             ` | `Auxiliary`
|        base        |    syntaxes    |  auxiliary-syntax  | `=>                              ` | `Auxiliary`
|        base        |    syntaxes    |  auxiliary-syntax  | `else                            ` | `Auxiliary`
|        base        |   quotation    |       syntax       | `quote                           ` | `PrimitiveV(Quote)`
|        base        |   quotation    |       syntax       | `quasiquote                      ` | `PrimitiveV(QuasiQuote)`
|        base        |   quotation    |       syntax       | `unquote                         ` | `PrimitiveV(UnQuote)`
|        base        |   quotation    |       syntax       | `unquote-splicing                ` | `PrimitiveV(UnQuoteSplicing)`
|        base        |    control     |       syntax       | `begin                           ` | `PrimitiveV(Begin)`
|        base        |    control     |       syntax       | `if                              ` | `PrimitiveV(If)`
|        base        |    control     |       syntax       | `unless                          ` | `PrimitiveV(Unless)`
|        base        |    control     |       syntax       | `when                            ` | `PrimitiveV(When)`
|        base        |    control     |       syntax       | `cond                            ` | `PrimitiveV(Cond)`
|        base        |    control     |       syntax       | `case                            ` | `PrimitiveV(Case)`
|        base        |    control     |       syntax       | `do                              ` | `PrimitiveV(Do)`
|        base        |    control     |       syntax       | `and                             ` | `PrimitiveV(And)`
|        base        |    control     |       syntax       | `or                              ` | `PrimitiveV(Or)`
|        base        |     lambda     |       syntax       | `lambda                          ` | `PrimitiveV(Lambda)`
|        base        |    contexts    |       syntax       | `define                          ` | `PrimitiveV(Define)`
|        base        |     values     |       syntax       | `define-values                   ` | `PrimitiveV(DefineValues)`
|        base        |    syntaxes    |       syntax       | `define-syntax                   ` | `Unsupported`
|        base        |    records     |       syntax       | `define-record-type              ` | `Unimplemented`
|        base        |    contexts    |       syntax       | `let                             ` | `PrimitiveV(LetParallel)`
|        base        |    contexts    |       syntax       | `let*                            ` | `PrimitiveV(LetSequential)`
|        base        |    contexts    |       syntax       | `letrec                          ` | `PrimitiveV(LetRecursiveParallel)`
|        base        |    contexts    |       syntax       | `letrec*                         ` | `PrimitiveV(LetRecursiveSequential)`
|        base        |     values     |       syntax       | `let-values                      ` | `PrimitiveV(LetValuesParallel)`
|        base        |     values     |       syntax       | `let*-values                     ` | `PrimitiveV(LetValuesSequential)`
|        base        |    syntaxes    |       syntax       | `let-syntax                      ` | `Unsupported`
|        base        |    syntaxes    |       syntax       | `letrec-syntax                   ` | `Unsupported`
|        base        |    contexts    |       syntax       | `set!                            ` | `PrimitiveV(Set)`
|        base        |    modules     |       syntax       | `import                          ` | `Unsupported`
|        base        |    modules     |       syntax       | `include                         ` | `Unsupported`
|        base        |    modules     |       syntax       | `include-ci                      ` | `Unsupported`
|        base        |    modules     |       syntax       | `cond-expand                     ` | `Unsupported`
|        base        |   parameters   |       syntax       | `parameterize                    ` | `Unsupported`
|        base        |   parameters   |     procedure      | `make-parameter                  ` | `Unsupported`
|        base        |    syntaxes    |       syntax       | `syntax-error                    ` | `Unsupported`
|        base        |    syntaxes    |       syntax       | `syntax-rules                    ` | `Unsupported`
|        base        |   evaluator    |       syntax       | `guard                           ` | `Unsupported`
|        base        |    modules     |     procedure      | `features                        ` | `Unsupported`
|        base        |     types      |    procedure-1     | `null?                           ` | `Primitive1(Type(IsNull))`
|        base        |  equivalence   |    procedure-1*    | `eq?                             ` | `Primitive1(Comparison(EquivalentByIdentity))`
|        base        |  equivalence   |    procedure-2*    | `eq?                             ` | `Primitive2(Comparison(EquivalentByIdentity))`
|        base        |  equivalence   |    procedure-3*    | `eq?                             ` | `Primitive3(Comparison(EquivalentByIdentity))`
|        base        |  equivalence   |    procedure-4*    | `eq?                             ` | `Primitive4(Comparison(EquivalentByIdentity))`
|        base        |  equivalence   |    procedure-n*    | `eq?                             ` | `PrimitiveN(Comparison(EquivalentByIdentity))`
|        base        |  equivalence   |    procedure-v*    | `eq?                             ` | `PrimitiveV(Comparison(EquivalentByIdentity))`
|        base        |  equivalence   |    procedure-1*    | `eqv?                            ` | `Primitive1(Comparison(EquivalentByValueStrict))`
|        base        |  equivalence   |    procedure-2*    | `eqv?                            ` | `Primitive2(Comparison(EquivalentByValueStrict))`
|        base        |  equivalence   |    procedure-3*    | `eqv?                            ` | `Primitive3(Comparison(EquivalentByValueStrict))`
|        base        |  equivalence   |    procedure-4*    | `eqv?                            ` | `Primitive4(Comparison(EquivalentByValueStrict))`
|        base        |  equivalence   |    procedure-n*    | `eqv?                            ` | `PrimitiveN(Comparison(EquivalentByValueStrict))`
|        base        |  equivalence   |    procedure-v*    | `eqv?                            ` | `PrimitiveV(Comparison(EquivalentByValueStrict))`
|        base        |  equivalence   |    procedure-1*    | `equal?                          ` | `Primitive1(Comparison(EquivalentByValueStrictRecursive))`
|        base        |  equivalence   |    procedure-2*    | `equal?                          ` | `Primitive2(Comparison(EquivalentByValueStrictRecursive))`
|        base        |  equivalence   |    procedure-3*    | `equal?                          ` | `Primitive3(Comparison(EquivalentByValueStrictRecursive))`
|        base        |  equivalence   |    procedure-4*    | `equal?                          ` | `Primitive4(Comparison(EquivalentByValueStrictRecursive))`
|        base        |  equivalence   |    procedure-n*    | `equal?                          ` | `PrimitiveN(Comparison(EquivalentByValueStrictRecursive))`
|        base        |  equivalence   |    procedure-v*    | `equal?                          ` | `PrimitiveV(Comparison(EquivalentByValueStrictRecursive))`
|        base        |     types      |    procedure-1     | `number?                         ` | `Primitive1(Type(IsNumber))`
|        base        |     types      |    procedure-1     | `integer?                        ` | `Primitive1(Type(IsNumberInteger))`
|        base        |     types      |    procedure-1     | `real?                           ` | `Primitive1(Type(IsNumberReal))`
|        base        |     types      |    procedure-1     | `rational?                       ` | `Primitive1(Type(IsNumberRational))`
|        base        |     types      |    procedure-1     | `complex?                        ` | `Primitive1(Type(IsNumberComplex))`
|        base        |     types      |    procedure-1     | `exact?                          ` | `Primitive1(Type(IsNumberExact))`
|        base        |     types      |    procedure-1     | `exact-integer?                  ` | `Primitive1(Type(IsNumberExactInteger))`
|        base        |     types      |    procedure-1     | `inexact?                        ` | `Primitive1(Type(IsNumberInexact))`
|        base        |   arithmetic   |    procedure-1     | `zero?                           ` | `Primitive1(Type(IsNumberZero))`
|        base        |   arithmetic   |    procedure-1     | `positive?                       ` | `Primitive1(Type(IsNumberPositive))`
|        base        |   arithmetic   |    procedure-1     | `negative?                       ` | `Primitive1(Type(IsNumberNegative))`
|        base        |   arithmetic   |    procedure-1     | `odd?                            ` | `Primitive1(Type(IsNumberOdd))`
|        base        |   arithmetic   |    procedure-1     | `even?                           ` | `Primitive1(Type(IsNumberEven))`
|        base        |   arithmetic   |    procedure-0*    | `+                               ` | `Primitive0(Arithmetic(Addition))`
|        base        |   arithmetic   |    procedure-1*    | `+                               ` | `Primitive1(Arithmetic(Addition))`
|        base        |   arithmetic   |    procedure-2*    | `+                               ` | `Primitive2(Arithmetic(Addition))`
|        base        |   arithmetic   |    procedure-n*    | `+                               ` | `PrimitiveN(Arithmetic(Addition))`
|        base        |   arithmetic   |    procedure-v*    | `+                               ` | `PrimitiveV(Arithmetic(Addition))`
|        base        |   arithmetic   |    procedure-1*    | `-                               ` | `Primitive1(Arithmetic(Subtraction))`
|        base        |   arithmetic   |    procedure-2*    | `-                               ` | `Primitive2(Arithmetic(Subtraction))`
|        base        |   arithmetic   |    procedure-n*    | `-                               ` | `PrimitiveN(Arithmetic(Subtraction))`
|        base        |   arithmetic   |    procedure-v*    | `-                               ` | `PrimitiveV(Arithmetic(Subtraction))`
|        base        |   arithmetic   |    procedure-0*    | `*                               ` | `Primitive0(Arithmetic(Multiplication))`
|        base        |   arithmetic   |    procedure-1*    | `*                               ` | `Primitive1(Arithmetic(Multiplication))`
|        base        |   arithmetic   |    procedure-2*    | `*                               ` | `Primitive2(Arithmetic(Multiplication))`
|        base        |   arithmetic   |    procedure-n*    | `*                               ` | `PrimitiveN(Arithmetic(Multiplication))`
|        base        |   arithmetic   |    procedure-v*    | `*                               ` | `PrimitiveV(Arithmetic(Multiplication))`
|        base        |   arithmetic   |    procedure-1*    | `/                               ` | `Primitive1(Arithmetic(Division))`
|        base        |   arithmetic   |    procedure-2*    | `/                               ` | `Primitive2(Arithmetic(Division))`
|        base        |   arithmetic   |    procedure-n*    | `/                               ` | `PrimitiveN(Arithmetic(Division))`
|        base        |   arithmetic   |    procedure-v*    | `/                               ` | `PrimitiveV(Arithmetic(Division))`
|        base        |   arithmetic   |    procedure-1     | `abs                             ` | `Primitive1(Arithmetic(Absolute))`
|        base        |   arithmetic   |    procedure-2     | `quotient                        ` | `Primitive2(Arithmetic(DivisionTruncateQuotient))`
|        base        |   arithmetic   |    procedure-2     | `remainder                       ` | `Primitive2(Arithmetic(DivisionTruncateRemainder))`
|        base        |   arithmetic   |    procedure-2     | `modulo                          ` | `Primitive2(Arithmetic(DivisionFloorRemainder))`
|        base        |   arithmetic   |    procedure-1     | `floor                           ` | `Primitive1(Arithmetic(Floor))`
|        base        |   arithmetic   |    procedure-1     | `ceiling                         ` | `Primitive1(Arithmetic(Ceiling))`
|        base        |   arithmetic   |    procedure-1     | `truncate                        ` | `Primitive1(Arithmetic(Truncate))`
|        base        |   arithmetic   |    procedure-1     | `round                           ` | `Primitive1(Arithmetic(Round))`
|        base        |   arithmetic   |     procedure      | `rationalize                     ` | `Unsupported`
|        base        |   arithmetic   |     procedure      | `numerator                       ` | `Unsupported`
|        base        |   arithmetic   |     procedure      | `denominator                     ` | `Unsupported`
|        base        |   arithmetic   |    procedure-2     | `floor/                          ` | `Primitive2(Arithmetic(DivisionFloor))`
|        base        |   arithmetic   |    procedure-2     | `floor-quotient                  ` | `Primitive2(Arithmetic(DivisionFloorQuotient))`
|        base        |   arithmetic   |    procedure-2     | `floor-remainder                 ` | `Primitive2(Arithmetic(DivisionFloorRemainder))`
|        base        |   arithmetic   |    procedure-2     | `truncate/                       ` | `Primitive2(Arithmetic(DivisionTruncate))`
|        base        |   arithmetic   |    procedure-2     | `truncate-quotient               ` | `Primitive2(Arithmetic(DivisionTruncateQuotient))`
|        base        |   arithmetic   |    procedure-2     | `truncate-remainder              ` | `Primitive2(Arithmetic(DivisionTruncateRemainder))`
|        base        |   arithmetic   |    procedure-1*    | `min                             ` | `Primitive1(Arithmetic(Minimum))`
|        base        |   arithmetic   |    procedure-2*    | `min                             ` | `Primitive2(Arithmetic(Minimum))`
|        base        |   arithmetic   |    procedure-n*    | `min                             ` | `PrimitiveN(Arithmetic(Minimum))`
|        base        |   arithmetic   |    procedure-v*    | `min                             ` | `PrimitiveV(Arithmetic(Minimum))`
|        base        |   arithmetic   |    procedure-1*    | `max                             ` | `Primitive1(Arithmetic(Maximum))`
|        base        |   arithmetic   |    procedure-2*    | `max                             ` | `Primitive2(Arithmetic(Maximum))`
|        base        |   arithmetic   |    procedure-n*    | `max                             ` | `PrimitiveN(Arithmetic(Maximum))`
|        base        |   arithmetic   |    procedure-v*    | `max                             ` | `PrimitiveV(Arithmetic(Maximum))`
|        base        |   arithmetic   |    procedure-1*    | `gcd                             ` | `Primitive1(Arithmetic(GreatestCommonDivisor))`
|        base        |   arithmetic   |    procedure-2*    | `gcd                             ` | `Primitive2(Arithmetic(GreatestCommonDivisor))`
|        base        |   arithmetic   |    procedure-n*    | `gcd                             ` | `PrimitiveN(Arithmetic(GreatestCommonDivisor))`
|        base        |   arithmetic   |    procedure-v*    | `gcd                             ` | `PrimitiveV(Arithmetic(GreatestCommonDivisor))`
|        base        |   arithmetic   |    procedure-1*    | `lcm                             ` | `Primitive1(Arithmetic(LeastCommonMultiple))`
|        base        |   arithmetic   |    procedure-2*    | `lcm                             ` | `Primitive2(Arithmetic(LeastCommonMultiple))`
|        base        |   arithmetic   |    procedure-n*    | `lcm                             ` | `PrimitiveN(Arithmetic(LeastCommonMultiple))`
|        base        |   arithmetic   |    procedure-v*    | `lcm                             ` | `PrimitiveV(Arithmetic(LeastCommonMultiple))`
|        base        |   arithmetic   |    procedure-2     | `expt                            ` | `Primitive2(Arithmetic(Power))`
|        base        |   arithmetic   |    procedure-1     | `square                          ` | `Primitive1(Arithmetic(Square))`
|        base        |   arithmetic   |    procedure-1     | `exact-integer-sqrt              ` | `Primitive1(Arithmetic(SquareRootWithRemainder))`
|        base        |   arithmetic   |    procedure-1*    | `=                               ` | `Primitive1(Comparison(NumberEqual))`
|        base        |   arithmetic   |    procedure-2*    | `=                               ` | `Primitive2(Comparison(NumberEqual))`
|        base        |   arithmetic   |    procedure-3*    | `=                               ` | `Primitive3(Comparison(NumberEqual))`
|        base        |   arithmetic   |    procedure-4*    | `=                               ` | `Primitive4(Comparison(NumberEqual))`
|        base        |   arithmetic   |    procedure-n*    | `=                               ` | `PrimitiveN(Comparison(NumberEqual))`
|        base        |   arithmetic   |    procedure-v*    | `=                               ` | `PrimitiveV(Comparison(NumberEqual))`
|        base        |   arithmetic   |    procedure-1*    | `<                               ` | `Primitive1(Comparison(NumberLesser))`
|        base        |   arithmetic   |    procedure-2*    | `<                               ` | `Primitive2(Comparison(NumberLesser))`
|        base        |   arithmetic   |    procedure-3*    | `<                               ` | `Primitive3(Comparison(NumberLesser))`
|        base        |   arithmetic   |    procedure-4*    | `<                               ` | `Primitive4(Comparison(NumberLesser))`
|        base        |   arithmetic   |    procedure-n*    | `<                               ` | `PrimitiveN(Comparison(NumberLesser))`
|        base        |   arithmetic   |    procedure-v*    | `<                               ` | `PrimitiveV(Comparison(NumberLesser))`
|        base        |   arithmetic   |    procedure-1*    | `>                               ` | `Primitive1(Comparison(NumberGreater))`
|        base        |   arithmetic   |    procedure-2*    | `>                               ` | `Primitive2(Comparison(NumberGreater))`
|        base        |   arithmetic   |    procedure-3*    | `>                               ` | `Primitive3(Comparison(NumberGreater))`
|        base        |   arithmetic   |    procedure-4*    | `>                               ` | `Primitive4(Comparison(NumberGreater))`
|        base        |   arithmetic   |    procedure-n*    | `>                               ` | `PrimitiveN(Comparison(NumberGreater))`
|        base        |   arithmetic   |    procedure-v*    | `>                               ` | `PrimitiveV(Comparison(NumberGreater))`
|        base        |   arithmetic   |    procedure-1*    | `<=                              ` | `Primitive1(Comparison(NumberLesserOrEqual))`
|        base        |   arithmetic   |    procedure-2*    | `<=                              ` | `Primitive2(Comparison(NumberLesserOrEqual))`
|        base        |   arithmetic   |    procedure-3*    | `<=                              ` | `Primitive3(Comparison(NumberLesserOrEqual))`
|        base        |   arithmetic   |    procedure-4*    | `<=                              ` | `Primitive4(Comparison(NumberLesserOrEqual))`
|        base        |   arithmetic   |    procedure-n*    | `<=                              ` | `PrimitiveN(Comparison(NumberLesserOrEqual))`
|        base        |   arithmetic   |    procedure-v*    | `<=                              ` | `PrimitiveV(Comparison(NumberLesserOrEqual))`
|        base        |   arithmetic   |    procedure-1*    | `>=                              ` | `Primitive1(Comparison(NumberGreaterOrEqual))`
|        base        |   arithmetic   |    procedure-2*    | `>=                              ` | `Primitive2(Comparison(NumberGreaterOrEqual))`
|        base        |   arithmetic   |    procedure-3*    | `>=                              ` | `Primitive3(Comparison(NumberGreaterOrEqual))`
|        base        |   arithmetic   |    procedure-4*    | `>=                              ` | `Primitive4(Comparison(NumberGreaterOrEqual))`
|        base        |   arithmetic   |    procedure-n*    | `>=                              ` | `PrimitiveN(Comparison(NumberGreaterOrEqual))`
|        base        |   arithmetic   |    procedure-v*    | `>=                              ` | `PrimitiveV(Comparison(NumberGreaterOrEqual))`
|        base        |   arithmetic   |    procedure-1     | `inexact                         ` | `Primitive1(Arithmetic(CoerceToInexact))`
|        base        |   arithmetic   |    procedure-1     | `exact                           ` | `Primitive1(Arithmetic(CoerceToExact))`
|        base        |     types      |    procedure-1     | `boolean?                        ` | `Primitive1(Type(IsBoolean))`
|        base        |  equivalence   |    procedure-1*    | `boolean=?                       ` | `Primitive1(Comparison(BooleanEqual))`
|        base        |  equivalence   |    procedure-2*    | `boolean=?                       ` | `Primitive2(Comparison(BooleanEqual))`
|        base        |  equivalence   |    procedure-3*    | `boolean=?                       ` | `Primitive3(Comparison(BooleanEqual))`
|        base        |  equivalence   |    procedure-4*    | `boolean=?                       ` | `Primitive4(Comparison(BooleanEqual))`
|        base        |  equivalence   |    procedure-n*    | `boolean=?                       ` | `PrimitiveN(Comparison(BooleanEqual))`
|        base        |  equivalence   |    procedure-v*    | `boolean=?                       ` | `PrimitiveV(Comparison(BooleanEqual))`
|        base        |  equivalence   |    procedure-1     | `not                             ` | `Primitive1(Type(IsFalse))`
|        base        |     types      |    procedure-1     | `char?                           ` | `Primitive1(Type(IsCharacter))`
|        base        |   characters   |    procedure-1*    | `char=?                          ` | `Primitive1(Comparison(CharacterCaseSensitiveEqual))`
|        base        |   characters   |    procedure-2*    | `char=?                          ` | `Primitive2(Comparison(CharacterCaseSensitiveEqual))`
|        base        |   characters   |    procedure-3*    | `char=?                          ` | `Primitive3(Comparison(CharacterCaseSensitiveEqual))`
|        base        |   characters   |    procedure-4*    | `char=?                          ` | `Primitive4(Comparison(CharacterCaseSensitiveEqual))`
|        base        |   characters   |    procedure-n*    | `char=?                          ` | `PrimitiveN(Comparison(CharacterCaseSensitiveEqual))`
|        base        |   characters   |    procedure-v*    | `char=?                          ` | `PrimitiveV(Comparison(CharacterCaseSensitiveEqual))`
|        base        |   characters   |    procedure-1*    | `char<?                          ` | `Primitive1(Comparison(CharacterCaseSensitiveLesser))`
|        base        |   characters   |    procedure-2*    | `char<?                          ` | `Primitive2(Comparison(CharacterCaseSensitiveLesser))`
|        base        |   characters   |    procedure-3*    | `char<?                          ` | `Primitive3(Comparison(CharacterCaseSensitiveLesser))`
|        base        |   characters   |    procedure-4*    | `char<?                          ` | `Primitive4(Comparison(CharacterCaseSensitiveLesser))`
|        base        |   characters   |    procedure-n*    | `char<?                          ` | `PrimitiveN(Comparison(CharacterCaseSensitiveLesser))`
|        base        |   characters   |    procedure-v*    | `char<?                          ` | `PrimitiveV(Comparison(CharacterCaseSensitiveLesser))`
|        base        |   characters   |    procedure-1*    | `char>?                          ` | `Primitive1(Comparison(CharacterCaseSensitiveGreater))`
|        base        |   characters   |    procedure-2*    | `char>?                          ` | `Primitive2(Comparison(CharacterCaseSensitiveGreater))`
|        base        |   characters   |    procedure-3*    | `char>?                          ` | `Primitive3(Comparison(CharacterCaseSensitiveGreater))`
|        base        |   characters   |    procedure-4*    | `char>?                          ` | `Primitive4(Comparison(CharacterCaseSensitiveGreater))`
|        base        |   characters   |    procedure-n*    | `char>?                          ` | `PrimitiveN(Comparison(CharacterCaseSensitiveGreater))`
|        base        |   characters   |    procedure-v*    | `char>?                          ` | `PrimitiveV(Comparison(CharacterCaseSensitiveGreater))`
|        base        |   characters   |    procedure-1*    | `char<=?                         ` | `Primitive1(Comparison(CharacterCaseSensitiveLesserOrEqual))`
|        base        |   characters   |    procedure-2*    | `char<=?                         ` | `Primitive2(Comparison(CharacterCaseSensitiveLesserOrEqual))`
|        base        |   characters   |    procedure-3*    | `char<=?                         ` | `Primitive3(Comparison(CharacterCaseSensitiveLesserOrEqual))`
|        base        |   characters   |    procedure-4*    | `char<=?                         ` | `Primitive4(Comparison(CharacterCaseSensitiveLesserOrEqual))`
|        base        |   characters   |    procedure-n*    | `char<=?                         ` | `PrimitiveN(Comparison(CharacterCaseSensitiveLesserOrEqual))`
|        base        |   characters   |    procedure-v*    | `char<=?                         ` | `PrimitiveV(Comparison(CharacterCaseSensitiveLesserOrEqual))`
|        base        |   characters   |    procedure-1*    | `char>=?                         ` | `Primitive1(Comparison(CharacterCaseSensitiveGreaterOrEqual))`
|        base        |   characters   |    procedure-2*    | `char>=?                         ` | `Primitive2(Comparison(CharacterCaseSensitiveGreaterOrEqual))`
|        base        |   characters   |    procedure-3*    | `char>=?                         ` | `Primitive3(Comparison(CharacterCaseSensitiveGreaterOrEqual))`
|        base        |   characters   |    procedure-4*    | `char>=?                         ` | `Primitive4(Comparison(CharacterCaseSensitiveGreaterOrEqual))`
|        base        |   characters   |    procedure-n*    | `char>=?                         ` | `PrimitiveN(Comparison(CharacterCaseSensitiveGreaterOrEqual))`
|        base        |   characters   |    procedure-v*    | `char>=?                         ` | `PrimitiveV(Comparison(CharacterCaseSensitiveGreaterOrEqual))`
|        base        |     types      |    procedure-1     | `symbol?                         ` | `Primitive1(Type(IsSymbol))`
|        base        |  equivalence   |    procedure-1*    | `symbol=?                        ` | `Primitive1(Comparison(SymbolCaseSensitiveEqual))`
|        base        |  equivalence   |    procedure-2*    | `symbol=?                        ` | `Primitive2(Comparison(SymbolCaseSensitiveEqual))`
|        base        |  equivalence   |    procedure-3*    | `symbol=?                        ` | `Primitive3(Comparison(SymbolCaseSensitiveEqual))`
|        base        |  equivalence   |    procedure-4*    | `symbol=?                        ` | `Primitive4(Comparison(SymbolCaseSensitiveEqual))`
|        base        |  equivalence   |    procedure-n*    | `symbol=?                        ` | `PrimitiveN(Comparison(SymbolCaseSensitiveEqual))`
|        base        |  equivalence   |    procedure-v*    | `symbol=?                        ` | `PrimitiveV(Comparison(SymbolCaseSensitiveEqual))`
|        base        |     types      |    procedure-1     | `pair?                           ` | `Primitive1(Type(IsPair))`
|        base        |     pairs      |    procedure-2     | `cons                            ` | `Primitive2(List(Pair))`
|        base        |     pairs      |    procedure-1     | `car                             ` | `Primitive1(List(PairLeft))`
|        base        |     pairs      |    procedure-1     | `cdr                             ` | `Primitive1(List(PairRight))`
|        base        |     pairs      |    procedure-2     | `set-car!                        ` | `Primitive2(List(PairLeftSet))`
|        base        |     pairs      |    procedure-2     | `set-cdr!                        ` | `Primitive2(List(PairRightSet))`
|        base        |     pairs      |    procedure-1     | `caar                            ` | `Primitive1(List(ListFirstOfFirst))`
|        base        |     pairs      |    procedure-1     | `cdar                            ` | `Primitive1(List(ListRestOfFirst))`
|        base        |     pairs      |    procedure-1     | `cadr                            ` | `Primitive1(List(ListFirstAt2))`
|        base        |     pairs      |    procedure-1     | `cddr                            ` | `Primitive1(List(ListRestAt2))`
|        base        |     types      |    procedure-1     | `list?                           ` | `Primitive1(Type(IsListProperOrEmpty))`
|        base        |     lists      |    procedure-0*    | `list                            ` | `Primitive0(List(ListBuild))`
|        base        |     lists      |    procedure-1*    | `list                            ` | `Primitive1(List(ListBuild))`
|        base        |     lists      |    procedure-2*    | `list                            ` | `Primitive2(List(ListBuild))`
|        base        |     lists      |    procedure-3*    | `list                            ` | `Primitive3(List(ListBuild))`
|        base        |     lists      |    procedure-4*    | `list                            ` | `Primitive4(List(ListBuild))`
|        base        |     lists      |    procedure-n*    | `list                            ` | `PrimitiveN(List(ListBuild))`
|        base        |     lists      |    procedure-v*    | `list                            ` | `PrimitiveV(List(ListBuild))`
|        base        |     lists      |    procedure-1*    | `make-list                       ` | `Primitive1(List(ListMake))`
|        base        |     lists      |    procedure-2*    | `make-list                       ` | `Primitive2(List(ListMake))`
|        base        |     lists      |    procedure-v*    | `make-list                       ` | `PrimitiveV(List(ListMake))`
|        base        |     lists      |    procedure-1*    | `list-copy                       ` | `Primitive1(List(ListClone))`
|        base        |     lists      |    procedure-2*    | `list-copy                       ` | `Primitive2(List(ListRangeClone))`
|        base        |     lists      |    procedure-3*    | `list-copy                       ` | `Primitive3(List(ListRangeClone))`
|        base        |     lists      |    procedure-v*    | `list-copy                       ` | `PrimitiveV(List(ListRangeClone))`
|        base        |     lists      |    procedure-0*    | `append                          ` | `Primitive0(List(ListAppend))`
|        base        |     lists      |    procedure-1*    | `append                          ` | `Primitive1(List(ListAppend))`
|        base        |     lists      |    procedure-2*    | `append                          ` | `Primitive2(List(ListAppend))`
|        base        |     lists      |    procedure-3*    | `append                          ` | `Primitive3(List(ListAppend))`
|        base        |     lists      |    procedure-4*    | `append                          ` | `Primitive4(List(ListAppend))`
|        base        |     lists      |    procedure-n*    | `append                          ` | `PrimitiveN(List(ListAppend))`
|        base        |     lists      |    procedure-v*    | `append                          ` | `PrimitiveV(List(ListAppend))`
|        base        |     lists      |    procedure-1     | `length                          ` | `Primitive1(List(ListLength))`
|        base        |     lists      |    procedure-2     | `list-ref                        ` | `Primitive2(List(ListFirstAt))`
|        base        |     lists      |    procedure-2     | `list-tail                       ` | `Primitive2(List(ListPairAt))`
|        base        |     lists      |    procedure-3     | `list-set!                       ` | `Primitive3(List(ListFirstAtSet))`
|        base        |     lists      |    procedure-1     | `reverse                         ` | `Primitive1(List(ListReverse))`
|        base        |     lists      |    procedure-2     | `memq                            ` | `Primitive2(List(ListMemberByIdentity))`
|        base        |     lists      |    procedure-2     | `memv                            ` | `Primitive2(List(ListMemberByValue))`
|        base        |     lists      |    procedure-2*    | `member                          ` | `Primitive2(List(ListMemberByValueRecursive))`
|        base        |     lists      |    procedure-3*    | `member                          ` | `Primitive3(List(ListMemberByComparator))`
|        base        |     lists      |    procedure-v*    | `member                          ` | `PrimitiveV(List(ListMember))`
|        base        |     lists      |    procedure-2     | `assq                            ` | `Primitive2(List(ListAssocByIdentity))`
|        base        |     lists      |    procedure-2     | `assv                            ` | `Primitive2(List(ListAssocByValue))`
|        base        |     lists      |    procedure-2*    | `assoc                           ` | `Primitive2(List(ListAssocByValueRecursive))`
|        base        |     lists      |    procedure-3*    | `assoc                           ` | `Primitive3(List(ListAssocByComparator))`
|        base        |     lists      |    procedure-v*    | `assoc                           ` | `PrimitiveV(List(ListAssoc))`
|        base        |     types      |    procedure-1     | `vector?                         ` | `Primitive1(Type(IsArray))`
|        base        |    vectors     |    procedure-0*    | `vector                          ` | `Primitive0(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-1*    | `vector                          ` | `Primitive1(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-2*    | `vector                          ` | `Primitive2(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-3*    | `vector                          ` | `Primitive3(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-4*    | `vector                          ` | `Primitive4(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-n*    | `vector                          ` | `PrimitiveN(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-v*    | `vector                          ` | `PrimitiveV(Array(ArrayBuild))`
|        base        |    vectors     |    procedure-1*    | `make-vector                     ` | `Primitive1(Array(ArrayMake))`
|        base        |    vectors     |    procedure-2*    | `make-vector                     ` | `Primitive2(Array(ArrayMake))`
|        base        |    vectors     |    procedure-v*    | `make-vector                     ` | `PrimitiveV(Array(ArrayMake))`
|        base        |    vectors     |    procedure-1*    | `vector-copy                     ` | `Primitive1(Array(ArrayClone))`
|        base        |    vectors     |    procedure-2*    | `vector-copy                     ` | `Primitive2(Array(ArrayRangeClone))`
|        base        |    vectors     |    procedure-3*    | `vector-copy                     ` | `Primitive3(Array(ArrayRangeClone))`
|        base        |    vectors     |    procedure-v*    | `vector-copy                     ` | `PrimitiveV(Array(ArrayRangeClone))`
|        base        |    vectors     |    procedure-0*    | `vector-append                   ` | `Primitive0(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-1*    | `vector-append                   ` | `Primitive1(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-2*    | `vector-append                   ` | `Primitive2(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-3*    | `vector-append                   ` | `Primitive3(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-4*    | `vector-append                   ` | `Primitive4(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-n*    | `vector-append                   ` | `PrimitiveN(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-v*    | `vector-append                   ` | `PrimitiveV(Array(ArrayAppend))`
|        base        |    vectors     |    procedure-1     | `vector-length                   ` | `Primitive1(Array(ArrayLength))`
|        base        |    vectors     |    procedure-2     | `vector-ref                      ` | `Primitive2(Array(ArrayAt))`
|        base        |    vectors     |    procedure-3     | `vector-set!                     ` | `Primitive3(Array(ArrayAtSet))`
|        base        |    vectors     |    procedure-1*    | `vector-fill!                    ` | `Primitive1(Array(ArrayFill))`
|        base        |    vectors     |    procedure-2*    | `vector-fill!                    ` | `Primitive2(Array(ArrayFill))`
|        base        |    vectors     |    procedure-3*    | `vector-fill!                    ` | `Primitive3(Array(ArrayRangeFill))`
|        base        |    vectors     |    procedure-4*    | `vector-fill!                    ` | `Primitive4(Array(ArrayRangeFill))`
|        base        |    vectors     |    procedure-v*    | `vector-fill!                    ` | `PrimitiveV(Array(ArrayRangeFill))`
|        base        |    vectors     |    procedure-2*    | `vector-copy!                    ` | `Primitive2(Array(ArrayCopy))`
|        base        |    vectors     |    procedure-3*    | `vector-copy!                    ` | `Primitive3(Array(ArrayRangeCopy))`
|        base        |    vectors     |    procedure-4*    | `vector-copy!                    ` | `Primitive4(Array(ArrayRangeCopy))`
|        base        |    vectors     |    procedure-5*    | `vector-copy!                    ` | `Primitive5(Array(ArrayRangeCopy))`
|        base        |    vectors     |    procedure-v*    | `vector-copy!                    ` | `PrimitiveV(Array(ArrayRangeCopy))`
|        base        |     types      |    procedure-1     | `bytevector?                     ` | `Primitive1(Type(IsBytes))`
|        base        |     bytes      |    procedure-0*    | `bytevector                      ` | `Primitive0(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-1*    | `bytevector                      ` | `Primitive1(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-2*    | `bytevector                      ` | `Primitive2(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-3*    | `bytevector                      ` | `Primitive3(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-4*    | `bytevector                      ` | `Primitive4(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-n*    | `bytevector                      ` | `PrimitiveN(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-v*    | `bytevector                      ` | `PrimitiveV(Bytes(BytesBuild))`
|        base        |     bytes      |    procedure-1*    | `make-bytevector                 ` | `Primitive1(Bytes(BytesMake))`
|        base        |     bytes      |    procedure-2*    | `make-bytevector                 ` | `Primitive2(Bytes(BytesMake))`
|        base        |     bytes      |    procedure-v*    | `make-bytevector                 ` | `PrimitiveV(Bytes(BytesMake))`
|        base        |     bytes      |    procedure-1*    | `bytevector-copy                 ` | `Primitive1(Bytes(BytesClone))`
|        base        |     bytes      |    procedure-2*    | `bytevector-copy                 ` | `Primitive2(Bytes(BytesRangeClone))`
|        base        |     bytes      |    procedure-3*    | `bytevector-copy                 ` | `Primitive3(Bytes(BytesRangeClone))`
|        base        |     bytes      |    procedure-v*    | `bytevector-copy                 ` | `PrimitiveV(Bytes(BytesRangeClone))`
|        base        |     bytes      |    procedure-0*    | `bytevector-append               ` | `Primitive0(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-1*    | `bytevector-append               ` | `Primitive1(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-2*    | `bytevector-append               ` | `Primitive2(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-3*    | `bytevector-append               ` | `Primitive3(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-4*    | `bytevector-append               ` | `Primitive4(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-n*    | `bytevector-append               ` | `PrimitiveN(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-v*    | `bytevector-append               ` | `PrimitiveV(Bytes(BytesAppend))`
|        base        |     bytes      |    procedure-1     | `bytevector-length               ` | `Primitive1(Bytes(BytesLength))`
|        base        |     bytes      |    procedure-2     | `bytevector-u8-ref               ` | `Primitive2(Bytes(BytesAt))`
|        base        |     bytes      |    procedure-3     | `bytevector-u8-set!              ` | `Primitive3(Bytes(BytesAtSet))`
|        base        |     bytes      |    procedure-2*    | `bytevector-copy!                ` | `Primitive2(Bytes(BytesCopy))`
|        base        |     bytes      |    procedure-3*    | `bytevector-copy!                ` | `Primitive3(Bytes(BytesRangeCopy))`
|        base        |     bytes      |    procedure-4*    | `bytevector-copy!                ` | `Primitive4(Bytes(BytesRangeCopy))`
|        base        |     bytes      |    procedure-5*    | `bytevector-copy!                ` | `Primitive5(Bytes(BytesRangeCopy))`
|        base        |     bytes      |    procedure-v*    | `bytevector-copy!                ` | `PrimitiveV(Bytes(BytesRangeCopy))`
|        base        |     types      |    procedure-1     | `string?                         ` | `Primitive1(Type(IsString))`
|        base        |    strings     |    procedure-0*    | `string                          ` | `Primitive0(String(StringBuild))`
|        base        |    strings     |    procedure-1*    | `string                          ` | `Primitive1(String(StringBuild))`
|        base        |    strings     |    procedure-2*    | `string                          ` | `Primitive2(String(StringBuild))`
|        base        |    strings     |    procedure-3*    | `string                          ` | `Primitive3(String(StringBuild))`
|        base        |    strings     |    procedure-4*    | `string                          ` | `Primitive4(String(StringBuild))`
|        base        |    strings     |    procedure-n*    | `string                          ` | `PrimitiveN(String(StringBuild))`
|        base        |    strings     |    procedure-v*    | `string                          ` | `PrimitiveV(String(StringBuild))`
|        base        |    strings     |    procedure-1*    | `make-string                     ` | `Primitive1(String(StringMake))`
|        base        |    strings     |    procedure-2*    | `make-string                     ` | `Primitive2(String(StringMake))`
|        base        |    strings     |    procedure-v*    | `make-string                     ` | `PrimitiveV(String(StringMake))`
|        base        |    strings     |    procedure-1*    | `string-copy                     ` | `Primitive1(String(StringClone))`
|        base        |    strings     |    procedure-2*    | `string-copy                     ` | `Primitive2(String(StringRangeClone))`
|        base        |    strings     |    procedure-3*    | `string-copy                     ` | `Primitive3(String(StringRangeClone))`
|        base        |    strings     |    procedure-v*    | `string-copy                     ` | `PrimitiveV(String(StringRangeClone))`
|        base        |    strings     |    procedure-0*    | `string-append                   ` | `Primitive0(String(StringAppend))`
|        base        |    strings     |    procedure-1*    | `string-append                   ` | `Primitive1(String(StringAppend))`
|        base        |    strings     |    procedure-2*    | `string-append                   ` | `Primitive2(String(StringAppend))`
|        base        |    strings     |    procedure-3*    | `string-append                   ` | `Primitive3(String(StringAppend))`
|        base        |    strings     |    procedure-4*    | `string-append                   ` | `Primitive4(String(StringAppend))`
|        base        |    strings     |    procedure-n*    | `string-append                   ` | `PrimitiveN(String(StringAppend))`
|        base        |    strings     |    procedure-v*    | `string-append                   ` | `PrimitiveV(String(StringAppend))`
|        base        |    strings     |    procedure-1     | `string-length                   ` | `Primitive1(String(StringLength))`
|        base        |    strings     |    procedure-2     | `string-ref                      ` | `Primitive2(String(StringAt))`
|        base        |    strings     |    procedure-3     | `string-set!                     ` | `Primitive3(String(StringAtSet))`
|        base        |    strings     |    procedure-1*    | `string-fill!                    ` | `Primitive1(String(StringFill))`
|        base        |    strings     |    procedure-2*    | `string-fill!                    ` | `Primitive2(String(StringFill))`
|        base        |    strings     |    procedure-3*    | `string-fill!                    ` | `Primitive3(String(StringRangeFill))`
|        base        |    strings     |    procedure-4*    | `string-fill!                    ` | `Primitive4(String(StringRangeFill))`
|        base        |    strings     |    procedure-v*    | `string-fill!                    ` | `PrimitiveV(String(StringRangeFill))`
|        base        |    strings     |    procedure-2*    | `string-copy!                    ` | `Primitive2(String(StringCopy))`
|        base        |    strings     |    procedure-3*    | `string-copy!                    ` | `Primitive3(String(StringRangeCopy))`
|        base        |    strings     |    procedure-4*    | `string-copy!                    ` | `Primitive4(String(StringRangeCopy))`
|        base        |    strings     |    procedure-5*    | `string-copy!                    ` | `Primitive5(String(StringRangeCopy))`
|        base        |    strings     |    procedure-v*    | `string-copy!                    ` | `PrimitiveV(String(StringRangeCopy))`
|        base        |    strings     |    procedure-1*    | `substring                       ` | `Primitive1(String(StringClone))`
|        base        |    strings     |    procedure-2*    | `substring                       ` | `Primitive2(String(StringRangeClone))`
|        base        |    strings     |    procedure-3*    | `substring                       ` | `Primitive3(String(StringRangeClone))`
|        base        |    strings     |    procedure-v*    | `substring                       ` | `PrimitiveV(String(StringRangeClone))`
|        base        |    strings     |    procedure-1*    | `string=?                        ` | `Primitive1(Comparison(StringCaseSensitiveEqual))`
|        base        |    strings     |    procedure-2*    | `string=?                        ` | `Primitive2(Comparison(StringCaseSensitiveEqual))`
|        base        |    strings     |    procedure-3*    | `string=?                        ` | `Primitive3(Comparison(StringCaseSensitiveEqual))`
|        base        |    strings     |    procedure-4*    | `string=?                        ` | `Primitive4(Comparison(StringCaseSensitiveEqual))`
|        base        |    strings     |    procedure-n*    | `string=?                        ` | `PrimitiveN(Comparison(StringCaseSensitiveEqual))`
|        base        |    strings     |    procedure-v*    | `string=?                        ` | `PrimitiveV(Comparison(StringCaseSensitiveEqual))`
|        base        |    strings     |    procedure-1*    | `string<?                        ` | `Primitive1(Comparison(StringCaseSensitiveLesser))`
|        base        |    strings     |    procedure-2*    | `string<?                        ` | `Primitive2(Comparison(StringCaseSensitiveLesser))`
|        base        |    strings     |    procedure-3*    | `string<?                        ` | `Primitive3(Comparison(StringCaseSensitiveLesser))`
|        base        |    strings     |    procedure-4*    | `string<?                        ` | `Primitive4(Comparison(StringCaseSensitiveLesser))`
|        base        |    strings     |    procedure-n*    | `string<?                        ` | `PrimitiveN(Comparison(StringCaseSensitiveLesser))`
|        base        |    strings     |    procedure-v*    | `string<?                        ` | `PrimitiveV(Comparison(StringCaseSensitiveLesser))`
|        base        |    strings     |    procedure-1*    | `string>?                        ` | `Primitive1(Comparison(StringCaseSensitiveGreater))`
|        base        |    strings     |    procedure-2*    | `string>?                        ` | `Primitive2(Comparison(StringCaseSensitiveGreater))`
|        base        |    strings     |    procedure-3*    | `string>?                        ` | `Primitive3(Comparison(StringCaseSensitiveGreater))`
|        base        |    strings     |    procedure-4*    | `string>?                        ` | `Primitive4(Comparison(StringCaseSensitiveGreater))`
|        base        |    strings     |    procedure-n*    | `string>?                        ` | `PrimitiveN(Comparison(StringCaseSensitiveGreater))`
|        base        |    strings     |    procedure-v*    | `string>?                        ` | `PrimitiveV(Comparison(StringCaseSensitiveGreater))`
|        base        |    strings     |    procedure-1*    | `string<=?                       ` | `Primitive1(Comparison(StringCaseSensitiveLesserOrEqual))`
|        base        |    strings     |    procedure-2*    | `string<=?                       ` | `Primitive2(Comparison(StringCaseSensitiveLesserOrEqual))`
|        base        |    strings     |    procedure-3*    | `string<=?                       ` | `Primitive3(Comparison(StringCaseSensitiveLesserOrEqual))`
|        base        |    strings     |    procedure-4*    | `string<=?                       ` | `Primitive4(Comparison(StringCaseSensitiveLesserOrEqual))`
|        base        |    strings     |    procedure-n*    | `string<=?                       ` | `PrimitiveN(Comparison(StringCaseSensitiveLesserOrEqual))`
|        base        |    strings     |    procedure-v*    | `string<=?                       ` | `PrimitiveV(Comparison(StringCaseSensitiveLesserOrEqual))`
|        base        |    strings     |    procedure-1*    | `string>=?                       ` | `Primitive1(Comparison(StringCaseSensitiveGreaterOrEqual))`
|        base        |    strings     |    procedure-2*    | `string>=?                       ` | `Primitive2(Comparison(StringCaseSensitiveGreaterOrEqual))`
|        base        |    strings     |    procedure-3*    | `string>=?                       ` | `Primitive3(Comparison(StringCaseSensitiveGreaterOrEqual))`
|        base        |    strings     |    procedure-4*    | `string>=?                       ` | `Primitive4(Comparison(StringCaseSensitiveGreaterOrEqual))`
|        base        |    strings     |    procedure-n*    | `string>=?                       ` | `PrimitiveN(Comparison(StringCaseSensitiveGreaterOrEqual))`
|        base        |    strings     |    procedure-v*    | `string>=?                       ` | `PrimitiveV(Comparison(StringCaseSensitiveGreaterOrEqual))`
|        base        |    strings     |    procedure-1*    | `number->string                  ` | `Primitive1(String(NumberToString))`
|        base        |    strings     |    procedure-2*    | `number->string                  ` | `Primitive2(String(NumberToString))`
|        base        |    strings     |    procedure-3*    | `number->string                  ` | `Primitive3(String(NumberToString))`
|        base        |    strings     |    procedure-v*    | `number->string                  ` | `PrimitiveV(String(NumberToString))`
|        base        |    strings     |    procedure-1*    | `string->number                  ` | `Primitive1(String(StringToNumber))`
|        base        |    strings     |    procedure-2*    | `string->number                  ` | `Primitive2(String(StringToNumber))`
|        base        |    strings     |    procedure-v*    | `string->number                  ` | `PrimitiveV(String(StringToNumber))`
|        base        |    strings     |    procedure-1     | `symbol->string                  ` | `Primitive1(String(SymbolToString))`
|        base        |    strings     |    procedure-1     | `string->symbol                  ` | `Primitive1(String(StringToSymbol))`
|        base        |    strings     |    procedure-1*    | `list->string                    ` | `Primitive1(String(ListToString))`
|        base        |    strings     |    procedure-2*    | `list->string                    ` | `Primitive2(String(ListRangeToString))`
|        base        |    strings     |    procedure-3*    | `list->string                    ` | `Primitive3(String(ListRangeToString))`
|        base        |    strings     |    procedure-v*    | `list->string                    ` | `PrimitiveV(String(ListRangeToString))`
|        base        |    strings     |    procedure-1*    | `string->list                    ` | `Primitive1(String(StringToList))`
|        base        |    strings     |    procedure-2*    | `string->list                    ` | `Primitive2(String(StringRangeToList))`
|        base        |    strings     |    procedure-3*    | `string->list                    ` | `Primitive3(String(StringRangeToList))`
|        base        |    strings     |    procedure-v*    | `string->list                    ` | `PrimitiveV(String(StringRangeToList))`
|        base        |    strings     |    procedure-1*    | `utf8->string                    ` | `Primitive1(String(BytesToString))`
|        base        |    strings     |    procedure-2*    | `utf8->string                    ` | `Primitive2(String(BytesRangeToString))`
|        base        |    strings     |    procedure-3*    | `utf8->string                    ` | `Primitive3(String(BytesRangeToString))`
|        base        |    strings     |    procedure-v*    | `utf8->string                    ` | `PrimitiveV(String(BytesRangeToString))`
|        base        |    strings     |    procedure-1*    | `string->utf8                    ` | `Primitive1(String(StringToBytes))`
|        base        |    strings     |    procedure-2*    | `string->utf8                    ` | `Primitive2(String(StringRangeToBytes))`
|        base        |    strings     |    procedure-3*    | `string->utf8                    ` | `Primitive3(String(StringRangeToBytes))`
|        base        |    strings     |    procedure-v*    | `string->utf8                    ` | `PrimitiveV(String(StringRangeToBytes))`
|        base        |    strings     |    procedure-1*    | `vector->string                  ` | `Primitive1(String(ArrayToString))`
|        base        |    strings     |    procedure-2*    | `vector->string                  ` | `Primitive2(String(ArrayRangeToString))`
|        base        |    strings     |    procedure-3*    | `vector->string                  ` | `Primitive3(String(ArrayRangeToString))`
|        base        |    strings     |    procedure-v*    | `vector->string                  ` | `PrimitiveV(String(ArrayRangeToString))`
|        base        |    strings     |    procedure-1*    | `string->vector                  ` | `Primitive1(String(StringToArray))`
|        base        |    strings     |    procedure-2*    | `string->vector                  ` | `Primitive2(String(StringRangeToArray))`
|        base        |    strings     |    procedure-3*    | `string->vector                  ` | `Primitive3(String(StringRangeToArray))`
|        base        |    strings     |    procedure-v*    | `string->vector                  ` | `PrimitiveV(String(StringRangeToArray))`
|        base        |   characters   |    procedure-1     | `char->integer                   ` | `Primitive1(String(CharacterToNumber))`
|        base        |   characters   |    procedure-1     | `integer->char                   ` | `Primitive1(String(NumberToCharacter))`
|        base        |    vectors     |    procedure-1*    | `vector->list                    ` | `Primitive1(Array(ArrayToList))`
|        base        |    vectors     |    procedure-2*    | `vector->list                    ` | `Primitive2(Array(ArrayRangeToList))`
|        base        |    vectors     |    procedure-3*    | `vector->list                    ` | `Primitive3(Array(ArrayRangeToList))`
|        base        |    vectors     |    procedure-v*    | `vector->list                    ` | `PrimitiveV(Array(ArrayRangeToList))`
|        base        |    vectors     |    procedure-1*    | `list->vector                    ` | `Primitive1(Array(ListToArray))`
|        base        |    vectors     |    procedure-2*    | `list->vector                    ` | `Primitive2(Array(ListRangeToArray))`
|        base        |    vectors     |    procedure-3*    | `list->vector                    ` | `Primitive3(Array(ListRangeToArray))`
|        base        |    vectors     |    procedure-v*    | `list->vector                    ` | `PrimitiveV(Array(ListRangeToArray))`
|        base        |     types      |    procedure-1     | `procedure?                      ` | `Primitive1(Type(IsProcedure))`
|        base        |   functions    |    procedure-1*    | `apply                           ` | `Primitive1(Functions(Apply))`
|        base        |   functions    |    procedure-2*    | `apply                           ` | `Primitive2(Functions(Apply))`
|        base        |   functions    |    procedure-3*    | `apply                           ` | `Primitive3(Functions(Apply))`
|        base        |   functions    |    procedure-4*    | `apply                           ` | `Primitive4(Functions(Apply))`
|        base        |   functions    |    procedure-5*    | `apply                           ` | `Primitive5(Functions(Apply))`
|        base        |   functions    |    procedure-n*    | `apply                           ` | `PrimitiveN(Functions(Apply))`
|        base        |   functions    |    procedure-v*    | `apply                           ` | `PrimitiveV(Functions(Apply))`
|        base        |   functions    |    procedure-2*    | `map                             ` | `Primitive2(Functions(ListsMap))`
|        base        |   functions    |    procedure-3*    | `map                             ` | `Primitive3(Functions(ListsMap))`
|        base        |   functions    |    procedure-4*    | `map                             ` | `Primitive4(Functions(ListsMap))`
|        base        |   functions    |    procedure-5*    | `map                             ` | `Primitive5(Functions(ListsMap))`
|        base        |   functions    |    procedure-n*    | `map                             ` | `PrimitiveN(Functions(ListsMap))`
|        base        |   functions    |    procedure-v*    | `map                             ` | `PrimitiveV(Functions(ListsMap))`
|        base        |   functions    |    procedure-2*    | `for-each                        ` | `Primitive2(Functions(ListsIterate))`
|        base        |   functions    |    procedure-3*    | `for-each                        ` | `Primitive3(Functions(ListsIterate))`
|        base        |   functions    |    procedure-4*    | `for-each                        ` | `Primitive4(Functions(ListsIterate))`
|        base        |   functions    |    procedure-5*    | `for-each                        ` | `Primitive5(Functions(ListsIterate))`
|        base        |   functions    |    procedure-n*    | `for-each                        ` | `PrimitiveN(Functions(ListsIterate))`
|        base        |   functions    |    procedure-v*    | `for-each                        ` | `PrimitiveV(Functions(ListsIterate))`
|        base        |   functions    |    procedure-2*    | `vector-map                      ` | `Primitive2(Functions(ArraysMap))`
|        base        |   functions    |    procedure-3*    | `vector-map                      ` | `Primitive3(Functions(ArraysMap))`
|        base        |   functions    |    procedure-4*    | `vector-map                      ` | `Primitive4(Functions(ArraysMap))`
|        base        |   functions    |    procedure-5*    | `vector-map                      ` | `Primitive5(Functions(ArraysMap))`
|        base        |   functions    |    procedure-n*    | `vector-map                      ` | `PrimitiveN(Functions(ArraysMap))`
|        base        |   functions    |    procedure-v*    | `vector-map                      ` | `PrimitiveV(Functions(ArraysMap))`
|        base        |   functions    |    procedure-2*    | `vector-for-each                 ` | `Primitive2(Functions(ArraysIterate))`
|        base        |   functions    |    procedure-3*    | `vector-for-each                 ` | `Primitive3(Functions(ArraysIterate))`
|        base        |   functions    |    procedure-4*    | `vector-for-each                 ` | `Primitive4(Functions(ArraysIterate))`
|        base        |   functions    |    procedure-5*    | `vector-for-each                 ` | `Primitive5(Functions(ArraysIterate))`
|        base        |   functions    |    procedure-n*    | `vector-for-each                 ` | `PrimitiveN(Functions(ArraysIterate))`
|        base        |   functions    |    procedure-v*    | `vector-for-each                 ` | `PrimitiveV(Functions(ArraysIterate))`
|        base        |   functions    |    procedure-2*    | `string-map                      ` | `Primitive2(Functions(StringsMap))`
|        base        |   functions    |    procedure-3*    | `string-map                      ` | `Primitive3(Functions(StringsMap))`
|        base        |   functions    |    procedure-4*    | `string-map                      ` | `Primitive4(Functions(StringsMap))`
|        base        |   functions    |    procedure-5*    | `string-map                      ` | `Primitive5(Functions(StringsMap))`
|        base        |   functions    |    procedure-n*    | `string-map                      ` | `PrimitiveN(Functions(StringsMap))`
|        base        |   functions    |    procedure-v*    | `string-map                      ` | `PrimitiveV(Functions(StringsMap))`
|        base        |   functions    |    procedure-2*    | `string-for-each                 ` | `Primitive2(Functions(StringsIterate))`
|        base        |   functions    |    procedure-3*    | `string-for-each                 ` | `Primitive3(Functions(StringsIterate))`
|        base        |   functions    |    procedure-4*    | `string-for-each                 ` | `Primitive4(Functions(StringsIterate))`
|        base        |   functions    |    procedure-5*    | `string-for-each                 ` | `Primitive5(Functions(StringsIterate))`
|        base        |   functions    |    procedure-n*    | `string-for-each                 ` | `PrimitiveN(Functions(StringsIterate))`
|        base        |   functions    |    procedure-v*    | `string-for-each                 ` | `PrimitiveV(Functions(StringsIterate))`
|        base        |     values     |    procedure-0*    | `values                          ` | `Primitive0(Functions(Values))`
|        base        |     values     |    procedure-1*    | `values                          ` | `Primitive1(Functions(Values))`
|        base        |     values     |    procedure-2*    | `values                          ` | `Primitive2(Functions(Values))`
|        base        |     values     |    procedure-3*    | `values                          ` | `Primitive3(Functions(Values))`
|        base        |     values     |    procedure-4*    | `values                          ` | `Primitive4(Functions(Values))`
|        base        |     values     |    procedure-n*    | `values                          ` | `PrimitiveN(Functions(Values))`
|        base        |     values     |    procedure-v*    | `values                          ` | `PrimitiveV(Functions(Values))`
|        base        |     values     |    procedure-2     | `call-with-values                ` | `Primitive2(Functions(CallWithValuesBuilder))`
|        base        |   evaluator    |     procedure      | `call-with-current-continuation  ` | `Unsupported`
|        base        |   evaluator    |     procedure      | `call/cc                         ` | `Unsupported`
|        base        |   evaluator    |     procedure      | `dynamic-wind                    ` | `Unsupported`
|        base        |   evaluator    |     procedure      | `raise                           ` | `Unimplemented`
|        base        |   evaluator    |     procedure      | `raise-continuable               ` | `Unsupported`
|        base        |     errors     |     procedure      | `error                           ` | `Unimplemented`
|        base        |     errors     |    procedure-1     | `error-object?                   ` | `Primitive1(Type(IsError))`
|        base        |     errors     |     procedure      | `error-object-message            ` | `Unimplemented`
|        base        |     errors     |     procedure      | `error-object-irritants          ` | `Unimplemented`
|        base        |     errors     |    procedure-1     | `read-error?                     ` | `Primitive1(Type(IsErrorPortInput))`
|        base        |     errors     |    procedure-1     | `file-error?                     ` | `Primitive1(Type(IsErrorFile))`
|        base        |   evaluator    |     procedure      | `with-exception-handler          ` | `Unimplemented`
|        base        |     ports      |    procedure-2     | `call-with-port                  ` | `Primitive2(Port(CallAndClose))`
|        base        |   parameters   |     procedure      | `current-input-port              ` | `Unsupported`
|        base        |   parameters   |     procedure      | `current-output-port             ` | `Unsupported`
|        base        |   parameters   |     procedure      | `current-error-port              ` | `Unsupported`
|        base        |     ports      |    procedure-1     | `port?                           ` | `Primitive1(Type(IsPort))`
|        base        |     ports      |    procedure-1     | `input-port?                     ` | `Primitive1(Type(IsPortInput))`
|        base        |     ports      |    procedure-1     | `input-port-open?                ` | `Primitive1(Port(IsInputOpen))`
|        base        |     ports      |    procedure-1     | `output-port?                    ` | `Primitive1(Type(IsPortOutput))`
|        base        |     ports      |    procedure-1     | `output-port-open?               ` | `Primitive1(Port(IsOutputOpen))`
|        base        |     ports      |    procedure-1     | `binary-port?                    ` | `Primitive1(Type(IsPortBinary))`
|        base        |     ports      |    procedure-1     | `textual-port?                   ` | `Primitive1(Type(IsPortTextual))`
|        base        |     ports      |    procedure-1     | `open-input-string               ` | `Primitive1(Port(InputFromString))`
|        base        |     ports      |    procedure-0     | `open-output-string              ` | `Primitive0(Port(OutputToString))`
|        base        |     ports      |    procedure-1     | `get-output-string               ` | `Primitive1(Port(OutputToStringFinalize))`
|        base        |     ports      |    procedure-1     | `open-input-bytevector           ` | `Primitive1(Port(InputFromBytes))`
|        base        |     ports      |    procedure-0     | `open-output-bytevector          ` | `Primitive0(Port(OutputToBytes))`
|        base        |     ports      |    procedure-1     | `get-output-bytevector           ` | `Primitive1(Port(OutputToBytesFinalize))`
|        base        |     ports      |    procedure-1     | `close-port                      ` | `Primitive1(Port(Close))`
|        base        |     ports      |    procedure-1     | `close-input-port                ` | `Primitive1(Port(CloseInput))`
|        base        |     ports      |    procedure-1     | `close-output-port               ` | `Primitive1(Port(CloseOutput))`
|        base        |     ports      |    procedure-1     | `char-ready?                     ` | `Primitive1(Port(CharacterReady))`
|        base        |     ports      |    procedure-1     | `peek-char                       ` | `Primitive1(Port(CharacterPeek))`
|        base        |     ports      |    procedure-1     | `read-char                       ` | `Primitive1(Port(CharacterRead))`
|        base        |     ports      |    procedure-2     | `read-string                     ` | `Primitive2(Port(StringReadCollect))`
|        base        |     ports      |    procedure-1     | `u8-ready?                       ` | `Primitive1(Port(ByteReady))`
|        base        |     ports      |    procedure-1     | `peek-u8                         ` | `Primitive1(Port(BytePeek))`
|        base        |     ports      |    procedure-1     | `read-u8                         ` | `Primitive1(Port(ByteRead))`
|        base        |     ports      |    procedure-2     | `read-bytevector                 ` | `Primitive2(Port(BytesReadCollect))`
|        base        |     ports      |    procedure-2     | `read-bytevector!                ` | `Primitive2(Port(BytesReadCopy))`
|        base        |     ports      |    procedure-1     | `read-line                       ` | `Primitive1(Port(StringReadLine))`
|        base        |     ports      |    procedure-0     | `eof-object                      ` | `Primitive0(Port(Eof))`
|        base        |     ports      |    procedure-1     | `eof-object?                     ` | `Primitive1(Type(IsPortEof))`
|        base        |     ports      |    procedure-2     | `write-char                      ` | `Primitive2(Port(CharacterWrite))`
|        base        |     ports      |    procedure-2     | `write-string                    ` | `Primitive2(Port(StringWrite))`
|        base        |     ports      |    procedure-2     | `write-u8                        ` | `Primitive2(Port(ByteWrite))`
|        base        |     ports      |    procedure-2     | `write-bytevector                ` | `Primitive2(Port(BytesWrite))`
|        base        |     ports      |    procedure-0     | `newline                         ` | `Primitive0(Port(RsNewLine))`
|        base        |     ports      |    procedure-1     | `flush-output-port               ` | `Primitive1(Port(FlushOutput))`
|    case-lambda     |     lambda     |       syntax       | `case-lambda                     ` | `Unsupported`
|        char        |    strings     |    procedure-1     | `string-upcase                   ` | `Primitive1(String(StringToUpperCase))`
|        char        |    strings     |    procedure-1     | `string-downcase                 ` | `Primitive1(String(StringToLowerCase))`
|        char        |    strings     |    procedure-1     | `string-foldcase                 ` | `Primitive1(String(StringToFoldCase))`
|        char        |    strings     |    procedure-1*    | `string-ci=?                     ` | `Primitive1(Comparison(StringCaseInsensitiveEqual))`
|        char        |    strings     |    procedure-2*    | `string-ci=?                     ` | `Primitive2(Comparison(StringCaseInsensitiveEqual))`
|        char        |    strings     |    procedure-3*    | `string-ci=?                     ` | `Primitive3(Comparison(StringCaseInsensitiveEqual))`
|        char        |    strings     |    procedure-4*    | `string-ci=?                     ` | `Primitive4(Comparison(StringCaseInsensitiveEqual))`
|        char        |    strings     |    procedure-n*    | `string-ci=?                     ` | `PrimitiveN(Comparison(StringCaseInsensitiveEqual))`
|        char        |    strings     |    procedure-v*    | `string-ci=?                     ` | `PrimitiveV(Comparison(StringCaseInsensitiveEqual))`
|        char        |    strings     |    procedure-1*    | `string-ci<?                     ` | `Primitive1(Comparison(StringCaseInsensitiveLesser))`
|        char        |    strings     |    procedure-2*    | `string-ci<?                     ` | `Primitive2(Comparison(StringCaseInsensitiveLesser))`
|        char        |    strings     |    procedure-3*    | `string-ci<?                     ` | `Primitive3(Comparison(StringCaseInsensitiveLesser))`
|        char        |    strings     |    procedure-4*    | `string-ci<?                     ` | `Primitive4(Comparison(StringCaseInsensitiveLesser))`
|        char        |    strings     |    procedure-n*    | `string-ci<?                     ` | `PrimitiveN(Comparison(StringCaseInsensitiveLesser))`
|        char        |    strings     |    procedure-v*    | `string-ci<?                     ` | `PrimitiveV(Comparison(StringCaseInsensitiveLesser))`
|        char        |    strings     |    procedure-1*    | `string-ci>?                     ` | `Primitive1(Comparison(StringCaseInsensitiveGreater))`
|        char        |    strings     |    procedure-2*    | `string-ci>?                     ` | `Primitive2(Comparison(StringCaseInsensitiveGreater))`
|        char        |    strings     |    procedure-3*    | `string-ci>?                     ` | `Primitive3(Comparison(StringCaseInsensitiveGreater))`
|        char        |    strings     |    procedure-4*    | `string-ci>?                     ` | `Primitive4(Comparison(StringCaseInsensitiveGreater))`
|        char        |    strings     |    procedure-n*    | `string-ci>?                     ` | `PrimitiveN(Comparison(StringCaseInsensitiveGreater))`
|        char        |    strings     |    procedure-v*    | `string-ci>?                     ` | `PrimitiveV(Comparison(StringCaseInsensitiveGreater))`
|        char        |    strings     |    procedure-1*    | `string-ci<=?                    ` | `Primitive1(Comparison(StringCaseInsensitiveLesserOrEqual))`
|        char        |    strings     |    procedure-2*    | `string-ci<=?                    ` | `Primitive2(Comparison(StringCaseInsensitiveLesserOrEqual))`
|        char        |    strings     |    procedure-3*    | `string-ci<=?                    ` | `Primitive3(Comparison(StringCaseInsensitiveLesserOrEqual))`
|        char        |    strings     |    procedure-4*    | `string-ci<=?                    ` | `Primitive4(Comparison(StringCaseInsensitiveLesserOrEqual))`
|        char        |    strings     |    procedure-n*    | `string-ci<=?                    ` | `PrimitiveN(Comparison(StringCaseInsensitiveLesserOrEqual))`
|        char        |    strings     |    procedure-v*    | `string-ci<=?                    ` | `PrimitiveV(Comparison(StringCaseInsensitiveLesserOrEqual))`
|        char        |    strings     |    procedure-1*    | `string-ci>=?                    ` | `Primitive1(Comparison(StringCaseInsensitiveGreaterOrEqual))`
|        char        |    strings     |    procedure-2*    | `string-ci>=?                    ` | `Primitive2(Comparison(StringCaseInsensitiveGreaterOrEqual))`
|        char        |    strings     |    procedure-3*    | `string-ci>=?                    ` | `Primitive3(Comparison(StringCaseInsensitiveGreaterOrEqual))`
|        char        |    strings     |    procedure-4*    | `string-ci>=?                    ` | `Primitive4(Comparison(StringCaseInsensitiveGreaterOrEqual))`
|        char        |    strings     |    procedure-n*    | `string-ci>=?                    ` | `PrimitiveN(Comparison(StringCaseInsensitiveGreaterOrEqual))`
|        char        |    strings     |    procedure-v*    | `string-ci>=?                    ` | `PrimitiveV(Comparison(StringCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-1     | `char-alphabetic?                ` | `Primitive1(Type(IsCharacterAlphabetic))`
|        char        |   characters   |    procedure-1     | `char-upper-case?                ` | `Primitive1(Type(IsCharacterAlphabeticUpperCase))`
|        char        |   characters   |    procedure-1     | `char-lower-case?                ` | `Primitive1(Type(IsCharacterAlphabeticLowerCase))`
|        char        |   characters   |    procedure-1     | `char-numeric?                   ` | `Primitive1(Type(IsCharacterNumeric))`
|        char        |   characters   |    procedure-1     | `char-whitespace?                ` | `Primitive1(Type(IsCharacterWhitespace))`
|        char        |   characters   |    procedure-1     | `char-upcase                     ` | `Primitive1(String(CharacterToUpperCase))`
|        char        |   characters   |    procedure-1     | `char-downcase                   ` | `Primitive1(String(CharacterToLowerCase))`
|        char        |   characters   |    procedure-1     | `char-foldcase                   ` | `Primitive1(String(CharacterToFoldCase))`
|        char        |   characters   |    procedure-1*    | `char-ci=?                       ` | `Primitive1(Comparison(CharacterCaseInsensitiveEqual))`
|        char        |   characters   |    procedure-2*    | `char-ci=?                       ` | `Primitive2(Comparison(CharacterCaseInsensitiveEqual))`
|        char        |   characters   |    procedure-3*    | `char-ci=?                       ` | `Primitive3(Comparison(CharacterCaseInsensitiveEqual))`
|        char        |   characters   |    procedure-4*    | `char-ci=?                       ` | `Primitive4(Comparison(CharacterCaseInsensitiveEqual))`
|        char        |   characters   |    procedure-n*    | `char-ci=?                       ` | `PrimitiveN(Comparison(CharacterCaseInsensitiveEqual))`
|        char        |   characters   |    procedure-v*    | `char-ci=?                       ` | `PrimitiveV(Comparison(CharacterCaseInsensitiveEqual))`
|        char        |   characters   |    procedure-1*    | `char-ci<?                       ` | `Primitive1(Comparison(CharacterCaseInsensitiveLesser))`
|        char        |   characters   |    procedure-2*    | `char-ci<?                       ` | `Primitive2(Comparison(CharacterCaseInsensitiveLesser))`
|        char        |   characters   |    procedure-3*    | `char-ci<?                       ` | `Primitive3(Comparison(CharacterCaseInsensitiveLesser))`
|        char        |   characters   |    procedure-4*    | `char-ci<?                       ` | `Primitive4(Comparison(CharacterCaseInsensitiveLesser))`
|        char        |   characters   |    procedure-n*    | `char-ci<?                       ` | `PrimitiveN(Comparison(CharacterCaseInsensitiveLesser))`
|        char        |   characters   |    procedure-v*    | `char-ci<?                       ` | `PrimitiveV(Comparison(CharacterCaseInsensitiveLesser))`
|        char        |   characters   |    procedure-1*    | `char-ci>?                       ` | `Primitive1(Comparison(CharacterCaseInsensitiveGreater))`
|        char        |   characters   |    procedure-2*    | `char-ci>?                       ` | `Primitive2(Comparison(CharacterCaseInsensitiveGreater))`
|        char        |   characters   |    procedure-3*    | `char-ci>?                       ` | `Primitive3(Comparison(CharacterCaseInsensitiveGreater))`
|        char        |   characters   |    procedure-4*    | `char-ci>?                       ` | `Primitive4(Comparison(CharacterCaseInsensitiveGreater))`
|        char        |   characters   |    procedure-n*    | `char-ci>?                       ` | `PrimitiveN(Comparison(CharacterCaseInsensitiveGreater))`
|        char        |   characters   |    procedure-v*    | `char-ci>?                       ` | `PrimitiveV(Comparison(CharacterCaseInsensitiveGreater))`
|        char        |   characters   |    procedure-1*    | `char-ci<=?                      ` | `Primitive1(Comparison(CharacterCaseInsensitiveLesserOrEqual))`
|        char        |   characters   |    procedure-2*    | `char-ci<=?                      ` | `Primitive2(Comparison(CharacterCaseInsensitiveLesserOrEqual))`
|        char        |   characters   |    procedure-3*    | `char-ci<=?                      ` | `Primitive3(Comparison(CharacterCaseInsensitiveLesserOrEqual))`
|        char        |   characters   |    procedure-4*    | `char-ci<=?                      ` | `Primitive4(Comparison(CharacterCaseInsensitiveLesserOrEqual))`
|        char        |   characters   |    procedure-n*    | `char-ci<=?                      ` | `PrimitiveN(Comparison(CharacterCaseInsensitiveLesserOrEqual))`
|        char        |   characters   |    procedure-v*    | `char-ci<=?                      ` | `PrimitiveV(Comparison(CharacterCaseInsensitiveLesserOrEqual))`
|        char        |   characters   |    procedure-1*    | `char-ci>=?                      ` | `Primitive1(Comparison(CharacterCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-2*    | `char-ci>=?                      ` | `Primitive2(Comparison(CharacterCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-3*    | `char-ci>=?                      ` | `Primitive3(Comparison(CharacterCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-4*    | `char-ci>=?                      ` | `Primitive4(Comparison(CharacterCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-n*    | `char-ci>=?                      ` | `PrimitiveN(Comparison(CharacterCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-v*    | `char-ci>=?                      ` | `PrimitiveV(Comparison(CharacterCaseInsensitiveGreaterOrEqual))`
|        char        |   characters   |    procedure-1*    | `digit-value                     ` | `Primitive1(String(CharacterToDigitNumber))`
|        char        |   characters   |    procedure-2*    | `digit-value                     ` | `Primitive2(String(CharacterToDigitNumber))`
|        char        |   characters   |    procedure-v*    | `digit-value                     ` | `PrimitiveV(String(CharacterToDigitNumber))`
|      complex       |   arithmetic   |     procedure      | `make-rectangular                ` | `Unsupported`
|      complex       |   arithmetic   |     procedure      | `real-part                       ` | `Unsupported`
|      complex       |   arithmetic   |     procedure      | `imag-part                       ` | `Unsupported`
|      complex       |   arithmetic   |     procedure      | `make-polar                      ` | `Unsupported`
|      complex       |   arithmetic   |     procedure      | `magnitude                       ` | `Unsupported`
|      complex       |   arithmetic   |     procedure      | `angle                           ` | `Unsupported`
|        cxr         |     pairs      |       value        | `caaar                           ` | `ComposedPrimitive1([List(PairLeft), List(PairLeft), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `caadr                           ` | `ComposedPrimitive1([List(PairLeft), List(PairLeft), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cadar                           ` | `ComposedPrimitive1([List(PairLeft), List(PairRight), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `caddr                           ` | `ComposedPrimitive1([List(PairLeft), List(PairRight), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cdaar                           ` | `ComposedPrimitive1([List(PairRight), List(PairLeft), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cdadr                           ` | `ComposedPrimitive1([List(PairRight), List(PairLeft), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cddar                           ` | `ComposedPrimitive1([List(PairRight), List(PairRight), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cdddr                           ` | `ComposedPrimitive1([List(PairRight), List(PairRight), List(PairRight)])`
|        cxr         |     pairs      |       value        | `caaaar                          ` | `ComposedPrimitive1([List(PairLeft), List(PairLeft), List(PairLeft), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `caaadr                          ` | `ComposedPrimitive1([List(PairLeft), List(PairLeft), List(PairLeft), List(PairRight)])`
|        cxr         |     pairs      |       value        | `caadar                          ` | `ComposedPrimitive1([List(PairLeft), List(PairLeft), List(PairRight), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `caaddr                          ` | `ComposedPrimitive1([List(PairLeft), List(PairLeft), List(PairRight), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cadaar                          ` | `ComposedPrimitive1([List(PairLeft), List(PairRight), List(PairLeft), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cadadr                          ` | `ComposedPrimitive1([List(PairLeft), List(PairRight), List(PairLeft), List(PairRight)])`
|        cxr         |     pairs      |       value        | `caddar                          ` | `ComposedPrimitive1([List(PairLeft), List(PairRight), List(PairRight), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cadddr                          ` | `ComposedPrimitive1([List(PairLeft), List(PairRight), List(PairRight), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cdaaar                          ` | `ComposedPrimitive1([List(PairRight), List(PairLeft), List(PairLeft), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cdaadr                          ` | `ComposedPrimitive1([List(PairRight), List(PairLeft), List(PairLeft), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cdadar                          ` | `ComposedPrimitive1([List(PairRight), List(PairLeft), List(PairRight), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cdaddr                          ` | `ComposedPrimitive1([List(PairRight), List(PairLeft), List(PairRight), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cddaar                          ` | `ComposedPrimitive1([List(PairRight), List(PairRight), List(PairLeft), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cddadr                          ` | `ComposedPrimitive1([List(PairRight), List(PairRight), List(PairLeft), List(PairRight)])`
|        cxr         |     pairs      |       value        | `cdddar                          ` | `ComposedPrimitive1([List(PairRight), List(PairRight), List(PairRight), List(PairLeft)])`
|        cxr         |     pairs      |       value        | `cddddr                          ` | `ComposedPrimitive1([List(PairRight), List(PairRight), List(PairRight), List(PairRight)])`
|        eval        |   evaluator    |     procedure      | `environment                     ` | `Unsupported`
|        eval        |   evaluator    |     procedure      | `eval                            ` | `Unsupported`
|        file        |     ports      |    procedure-1     | `open-input-file                 ` | `Primitive1(Port(OpenTextualInput))`
|        file        |     ports      |    procedure-1     | `open-binary-input-file          ` | `Primitive1(Port(OpenBinaryInput))`
|        file        |     ports      |    procedure-1     | `open-output-file                ` | `Primitive1(Port(OpenTextualOutput))`
|        file        |     ports      |    procedure-1     | `open-binary-output-file         ` | `Primitive1(Port(OpenBinaryOutput))`
|        file        |     ports      |    procedure-2     | `call-with-input-file            ` | `Primitive2(Port(OpenTextualInputThenCallAndClose))`
|        file        |     ports      |    procedure-2     | `call-with-output-file           ` | `Primitive2(Port(OpenTextualOutputThenCallAndClose))`
|        file        |   parameters   |     procedure      | `with-input-from-file            ` | `Unsupported`
|        file        |   parameters   |     procedure      | `with-output-to-file             ` | `Unsupported`
|        file        |     system     |    procedure-1     | `file-exists?                    ` | `Primitive1(Port(FileExists))`
|        file        |     system     |    procedure-1     | `delete-file                     ` | `Primitive1(Port(FileDelete))`
|      inexact       |   arithmetic   |    procedure-1     | `sqrt                            ` | `Primitive1(Arithmetic(SquareRoot))`
|      inexact       |   arithmetic   |    procedure-1     | `exp                             ` | `Primitive1(Arithmetic(Exponential))`
|      inexact       |   arithmetic   |    procedure-1     | `log                             ` | `Primitive1(Arithmetic(Logarithm))`
|      inexact       |   arithmetic   |    procedure-1     | `sin                             ` | `Primitive1(Arithmetic(Sin))`
|      inexact       |   arithmetic   |    procedure-1     | `cos                             ` | `Primitive1(Arithmetic(Cos))`
|      inexact       |   arithmetic   |    procedure-1     | `tan                             ` | `Primitive1(Arithmetic(Tan))`
|      inexact       |   arithmetic   |    procedure-1     | `asin                            ` | `Primitive1(Arithmetic(Asin))`
|      inexact       |   arithmetic   |    procedure-1     | `acos                            ` | `Primitive1(Arithmetic(Acos))`
|      inexact       |   arithmetic   |    procedure-1     | `atan                            ` | `Primitive1(Arithmetic(Atan))`
|      inexact       |   arithmetic   |    procedure-1     | `finite?                         ` | `Primitive1(Type(IsNumberFinite))`
|      inexact       |   arithmetic   |    procedure-1     | `infinite?                       ` | `Primitive1(Type(IsNumberInfinite))`
|      inexact       |   arithmetic   |    procedure-1     | `nan?                            ` | `Primitive1(Type(IsNumberNan))`
|        lazy        |    promises    |       syntax       | `delay                           ` | `Unsupported`
|        lazy        |    promises    |       syntax       | `delay-force                     ` | `Unsupported`
|        lazy        |    promises    |     procedure      | `promise?                        ` | `Unsupported`
|        lazy        |    promises    |     procedure      | `make-promise                    ` | `Unsupported`
|        lazy        |    promises    |     procedure      | `force                           ` | `Unsupported`
|        load        |    modules     |     procedure      | `load                            ` | `Unsupported`
|  process-context   |     system     |    procedure-0     | `command-line                    ` | `Primitive0(Runtime(ProcessArguments))`
|  process-context   |     system     |    procedure-1     | `get-environment-variable        ` | `Primitive1(Runtime(ProcessEnvironment))`
|  process-context   |     system     |    procedure-0     | `get-environment-variables       ` | `Primitive0(Runtime(ProcessEnvironment))`
|  process-context   |     system     |    procedure-0*    | `exit                            ` | `Primitive0(Runtime(ProcessExit))`
|  process-context   |     system     |    procedure-1*    | `exit                            ` | `Primitive1(Runtime(ProcessExit))`
|  process-context   |     system     |    procedure-v*    | `exit                            ` | `PrimitiveV(Runtime(ProcessExit))`
|  process-context   |     system     |    procedure-0*    | `emergency-exit                  ` | `Primitive0(Runtime(ProcessExitEmergency))`
|  process-context   |     system     |    procedure-1*    | `emergency-exit                  ` | `Primitive1(Runtime(ProcessExitEmergency))`
|  process-context   |     system     |    procedure-v*    | `emergency-exit                  ` | `PrimitiveV(Runtime(ProcessExitEmergency))`
|        r5rs        |   evaluator    |     procedure      | `interaction-environment         ` | `Unsupported`
|        r5rs        |   evaluator    |     procedure      | `scheme-report-environment       ` | `Unsupported`
|        r5rs        |   evaluator    |     procedure      | `null-environment                ` | `Unsupported`
|        read        |     ports      |    procedure-1     | `read                            ` | `Primitive1(Port(ValueRead))`
|        repl        |   evaluator    |     procedure      | `interaction-environment         ` | `Unsupported`
|        time        |     system     |    procedure-0     | `current-second                  ` | `Primitive0(Runtime(PosixTimestamp))`
|        time        |     system     |    procedure-0     | `current-jiffy                   ` | `Primitive0(Runtime(JiffiesTimestamp))`
|        time        |     system     |    procedure-0     | `jiffies-per-second              ` | `Primitive0(Runtime(JiffiesPerSecond))`
|       write        |     ports      |    procedure-2     | `write                           ` | `Primitive2(Port(ValueWrite))`
|       write        |     ports      |    procedure-2     | `write-shared                    ` | `Primitive2(Port(ValueWriteShared))`
|       write        |     ports      |    procedure-2     | `write-simple                    ` | `Primitive2(Port(ValueWriteSimple))`
|       write        |     ports      |    procedure-1     | `display                         ` | `Primitive1(Port(RsDisplay))`

****


## Scheme R7RS definitions -- summary

* implemented       289 (97.97% / 85.50%)

* unimplemented       6 (02.03% / 01.78%)
  ```
    define-record-type
    error
    error-object-irritants
    error-object-message
    raise
    with-exception-handler
  ```

* unsupported        43 (14.58% / 12.72%)
  ```
    angle
    call-with-current-continuation
    call/cc
    case-lambda
    cond-expand
    current-error-port
    current-input-port
    current-output-port
    define-syntax
    delay
    delay-force
    denominator
    dynamic-wind
    environment
    eval
    features
    force
    guard
    imag-part
    import
    include
    include-ci
    interaction-environment
    let-syntax
    letrec-syntax
    load
    magnitude
    make-parameter
    make-polar
    make-promise
    make-rectangular
    null-environment
    numerator
    parameterize
    promise?
    raise-continuable
    rationalize
    real-part
    scheme-report-environment
    syntax-error
    syntax-rules
    with-input-from-file
    with-output-to-file
  ```

* reserved            0 (00.00% / 00.00%)

****
