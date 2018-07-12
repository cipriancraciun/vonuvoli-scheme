

<a id='definition__r7rs__procedure_3f'></a>

# `procedure?` -- `r7rs` Definitions


<a id='definition__r7rs__procedure_3f__kind'></a>

#### Kind

`type-predicate`;


<a id='definition__r7rs__procedure_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((procedure) -> (true))`
   * input: a value of type [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__procedure_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base);


<a id='definition__r7rs__procedure_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme);


<a id='definition__r7rs__procedure_3f__description'></a>

#### Description

> ````
> (procedure? obj)
> ````
> 
> 
> Returns `#t` if `obj` is a procedure, otherwise returns `#f`.
> 
> ````
> (procedure? car)            ===>  #t
> (procedure? 'car)           ===>  #f
> (procedure? (lambda (x) (* x x)))
>                             ===>  #t
> (procedure? '(lambda (x) (* x x)))
>                             ===>  #f
> (call-with-current-continuation procedure?)
>                             ===>  #t
> ````
> 
> 
> ----
> > *The text herein was sourced and adapted as described in the ["R7RS attribution of various text snippets"](../../r7rs/appendices/attribution.md#appendix__r7rs__attribution) appendix.*


<a id='definition__r7rs__procedure_3f__referenced-types'></a>

#### Referenced-types

 * [`procedure`](../../r7rs/types/procedure.md#type__r7rs__procedure);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__procedure_3f__categories'></a>

#### Categories

 * [`vs:functions`](../../r7rs/categories/vs_3a_functions.md#category__r7rs__vs_3a_functions);
 * [`vs:types`](../../r7rs/categories/vs_3a_types.md#category__r7rs__vs_3a_types);


<a id='definition__r7rs__procedure_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices).

----
