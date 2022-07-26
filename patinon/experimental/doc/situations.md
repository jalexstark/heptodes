% Situations
% J. Alex Stark
% 2003--2022


----------------------------------------

Heptodes documents and other content in `doc` directories are licensed under the
[Creative Commons Attribution 4.0 License] (CC BY 4.0 license).

Source code licensed and code samples are licensed under the [Apache
2.0 License].

The CC BY 4.0 license requires attribution.  When samples, examples, figures,
tables, or other excerpts, are used in a tutorial, or a subdivision thereof, it
is sufficient to provide the complete source and license information once.  This
must be close to the beginning, such as in an early acknowledgments slide.  If
this is done, only short notes are required to be placed with each usage, such
as in figure captions.

[Creative Commons Attribution 4.0 License]: https://creativecommons.org/licenses/by/4.0/legalcode
[Apache 2.0 License]: https://www.apache.org/licenses/LICENSE-2.0

----------------------------------------


# Situations

## Criteria

*   As well as frequency and typestate differences, when thinking of returning a
    situation, consider: Do you want to burden the programmer with handling this
    situation at an early stage of program development?

### Situations {#situations}

**Braces item find moved to bottomup.**

> Item find with situation, alternative style.
>
> ```
> function ... |
>    <...>
>    loop i through [0..itemArray.size()]:
>       if (WantThisItem(itemArray[i])):
>          ==> ItemFound
>    |  |
>    < Handle item-not-found... >
> |== ItemFound:
>    < Handle situation using i... >
> |
> ```

#### Example, loop skip

Let us look at a function that iterates through a collection, and skips some
items, processing the others. The situations versions replace the `continue`
with a labelled situation.

As it stands, this example does not make sense. One would use an `if` / `else`
combination and assume that the compiler would merge the branches. However, in
reality the `continue` keyword is used in this fashion, such as where there is a
succession of reasons for skipping an item, where the reasons cannot be
evaluated in a single Boolean expression. The versions below that use situations
are slightly longer than the traditional `continue` version. When there is a
succession of reasons to skip, the situations would probably be given different
names. Then the programmer would see a summary at the end of a long stretch of
code showing all the reasons for skipping.

> Item skip, traditional control flow.
>
> ```
>    for (item : list_of_items) {
>       if (skipThis(item)) {
>          continue;
>       }
>       < Process item...>
>    }
> ```

> Item skip with situation, braces style.
>
> ```
>    for (item : list_of_items) {
>       if (skipThis(item)) {
>          ==> ItemNotRelevant
>       }
>       < Process item...>
>    } handle ItemNotRelevant {              // Empty handler.
>    }
> ```

> Item skip with situation, alternative style.
>
> ```
>    loop item [list_of_items] |
>       if (skipThis(item)):
>          ==> ItemNotRelevant
>       /
>       < Process item...>
>    |== ItemNotRelevant:                    ; Empty handler.
>    |
> ```

#### Further examples

More examples are given in the [Principal](principal.md#situation-examples) doc.

#### Fuller definition

In *Patinon* we expand support for situations to allow them to be used across
functions returns. Consider the C function `fopen()`, which returns a null
pointer if the request to open a file fails. In *Patinon* the file-open function
always creates a file structure, but there are two differences if the request
fails. First, the created structure will be in a different state (typestate),
with different fields being valid and accessible. Second, the function will
return via a *failure* situation that the caller will need to handle.

Let us look at a rough sketch of an example of file opening. Note that the part
that scans the file is very rough.

> ```
> fn ... ⌈
>    <...>
>    io.File config_file := io.FileOpen(src.filename)
>                                              ; This may trigger a Failed situation.
>
>    ;; Here the config_file object must be in the initialized / mutable typestate.
>    ;;    A call to config_file.ErrorNumber() would yield a compilte-time error.
>    loop ⌈                                    ; Some kind of file consumer.
>       line_text := config_file.ReadLine()    ; Validity of call guaranteed.
>    | while (...)
>    ⌋
> ⌋ == FileOpen::Failed ⌈
>    ;; Here the config_file object must be in the Failure typestate.
>    ;;    A call to config_file.ReadLine() would yield a compilte-time error.
>    switch (config_file.ErrorNumber()) ⌈      ; Validity of call guaranteed.
>    | case io.FileErrors.ENOENT               ; The action need not be specific to the error no.
>       < Report no such file...>
>    | default
>       < Report other error...>
>    ⌋
> ⌋
> ```

> The beginning of the `ErrorNumber()` function might look something like the
> following.

> ```
> fn ErrorNumber(ref self :File :error) -> error_num :FileErrors {
>    <...>
> }
> ```

That is to say that the `ReadLine` function is a method on a reference to a
`io.File` object that is in typestate `error` and returns a value of type
`io.FileErrors` that is named `error_num` within the function definition.

*This syntax is sketchy and preliminary. For example, it might be preferrable to
write a state or typestate as, say, `@error` instead of `error`. Uniformity of
syntax can be desirable, but it may also make code harder to read.*

The `io.File` *structure* has an off-the-shelf typestate graph, as we discuss
below. This has initialized and error (type) states. Reading data is only
possible in the initialized state, whereas the `ErrorNumber()` method can only
be applied in the Failure state. But this is not a problem when one has
typestate, and it is straightforward for *Patinon* to know that each of these
states is only possible in one of the branches of the bifurcation at the return
of the `io.FileOpen()` function.

This mechanism should be used sparingly. The critical benefit is this ability of
*Patinon* to trace a different execution pathway when a function returns with an
optional situation and the returned or modified data is in corresponding
typestates.

Let us summarize how we envisage the best use of situations. When Knuth argued
for situations in the early 1970s, he came to the realization that they were not
for handling of events or actions, even for exceptional events. Rather, they
should be used to deal with situations that occur. (See related
[core ideas](#wrestling).) We like this distinction, and explicitly recommend
that the decision to use a situation be based on frequency and data state. For
instance, the failure to open a file is a *normal* but *infrequent* occurrence
that the program should handle as ordinarily as it can.

*   Situations that span a short distance (from where they arise to where they
    are handled) will generally be on the less-frequent code pathway. When the
    span is not short, the situation should be infrequent.
*   Situations at a function return should typically correspond to an
    alternative typestate in mutated or returned data. This should also apply to
    situations within functions that span a medium distance.
*   Situations are not well-suited to exceptional events or errors, nor should
    they be used across very long spans or very deep nesting.

#### Additional notes

Let us touch briefly on exceptions and on error return values. We have made it
clear that situations are not intended to be used for exceptional events, so the
two are essentially different. On the other hand, we see situations and return
values as being somewhat complementary, and they can be used together. For
example, if the request to open a file should fail, it would be for one of a
number of reasons. The guidelines correctly suggest that we should not define
multiple situations, one for each reason. Instead, the reason would be contained
in the returned data. That does not mean that we would necessarily return a
separate error code in a tuple. Typestate allows us the option of having a
separate field within the file data structure.

If a routine makes more than one function call that returns a situation,
*Patinon* will need to distinguish between those. It will do through a
combination of automatic and explicit labels. For example, a routine might
request opening of an input file and then an output file. Either of these
requests might fail. What superficially appears to be an inconvenience proves in
reality to be an advantage. The handlers for input and output file opening
failures would lie close together with distinguishing labels.

## Situations

First read the [section on situations](principal.md#situations) in the
introduction. That gives an overview and some details about situations. We
expand a little here, considering some usage details and looking at some
examples of situations in current coding practice.

### Additional break example

Here is another example of usage of the `break` keyword. It is very similar to
the item-find example, except that the logic is inverted. Also, in this case we
might not need any code to process the results of the accumulation.

> Loop termination, traditional control flow.
>
> ```
> function ... {
>    <...>
>    bool done_early = false;
>    for (int i = 0; i < k; ++i) {
>       if (FinishNow(itemArray[i])) {
>          done_early = true;
>          break;
>       }
>       < Accumulate results... >
>    }
>    if (done_early) {
>       < Handle situation... >
>    } else {
>       < Process results... >
>    }
> }
> ```

> Loop termination, braces style.
>
> ```
> function ... {
>    for (int i = 0; i < k; ++i) {
>       if (FinishNow(itemArray[i])) {
>          ==> ProcessingDoneEarly
>       }
>       < Accumulate results... >
>    }
>    < Process results... >
> } handle ProcessingDoneEarly {
>    < Handle situation... >
> }
> ```

> Loop termination, alternative style.
>
> ```
> function ... :
>    loop i through [0..k]:
>       if (FinishNow(itemArray[i])):
>          ==> ProcessingDoneEarly
>       /
>       < Accumulate results... >
>    /
>    < Process results... >
> |== ProcessingDoneEarly:
>    < Handle situation... >
> /
> ```

### Wrestling over control in the 1970s {#wrestling}

(Moved to principal.)

We mentioned in the introduction the clear example of difficulty opening a file.
If a user specifies an input file that does not exist we should not think of
this as an event requiring exceptional handling, but rather leading to a
situation that we handle in an unexceptional fashion even though we expect it to
arise infrequently.

### Recent usage {situation-examples}

(Mostly moved to principal.)

GStreamer example: There is a problem with the simulated situations in this
example, and we will look at some usage guidelines shortly to deal with this.

```
function ... :
   <Set up data structure A>
   if (<Problem with structure A>):
      ==> DepartureScenario1
   /
   <Set up data structure B>
   if (<Problem detectable only after having B>):
      ==> DepartureScenario2
   /
   <Set up data structure C>

   < Core code for routine... >
```

In *Patinon*, we want to learn the lesson of the mistake of C's switch
statement. That is, continuing on by default and explicitly `break`-ing is a
material problem. This kind of design question is not like the choice of
material for a bike shed, but rather one that can lead to major security flaws.

Therefore *Patinon* will require explicit continuing-on, perhaps recycling the
`continue` keyword, so the function might be finished in the following fashion.

```
   < Tear down structure C>
   continue

|== DepartureScenario2:
   < Tear down structure B>
   continue

|== DepartureScenario1:
   < Tear down structure A>
/
```

### Design of situations

#### Basic design of situations

(Much of this copied to principal.)

In our examples of suggested uses of situations in *Patinon* we have largely
assumed a one-to-one correspondence of places in code where a situation is
raised and where it is handled. However, it is quite possible that *Patinon*
permit multiple lanuch points for one arrival point.

When situations were discussed forty years ago the formulations typically
required that all situations be declared in a list at the beginning of the block
structure, at the same level as the arrival points. We think it better that this
not be a requirement, since it would impede acceptance of situations. Brevity
tends to lead to clarity, and code exploration tools (and automatic
documentation tools) could annotate control structures with a list of their
situations.

#### Structuring situations

The GStreamer example, while correct, shows how situation handling could be
unclear and how errors might creep in. Specifically:

*   Situation handlers themselves raise situations in order to organize common
    code. The handlers for `no_filename` and `open_failed` finish by passing
    raising `error_exit`. The handlers for `no_stat`, `was_directory`,
    `was_socket` and `lseek_wonky` pass on to `error_close`.
*   These initial handlers are separate from each other, whereas an unwinding
    pattern is used with the second handlers: `error_close` continues on to
    `error_exit`.

The *Patinon* project will consider the best enforcement of situation usage.
This probably means starting out with a conservative approach. That means that
if situation handlers pass control via other situations, the primary situations
should be more deeply nested than the secondary situations. This would avoid
confusion of one situation cascading into another further down the list of
handlers. Also, *Patinon* should initially require that all situations within a
block either fully complete or all continue to the next. That is, separate
handlers should not be mixed with unwinding handlers. Once we have a range of
experimental code examples, *Patinon* will consider if these restrictions are
helpful and warranted.

### Other variations

*   One example is the *Go language* `defer` mechanism. There are some clear
    similarities with situations. However, we consider the differences
    important. Situations work well with typestate. A `defer` mechanism might
    nonetheless have its place. In this project, for the present, we focus on
    investigating situations.

## Typestate

As discussed in the [PRINCIPAL](principal.md), typestate is core to *Patinon*.
One might say that it is at the very heart. Contracts are a means of specifying
typestate across function boundaries. Situations express execution path
divergence and merging, in a manner that is simpler and more unified than
customary control flow. So, in a sense, *Patinon* is an experiment to see how
typestate can be made to work in practice, drawing on these other ideas.

### Integer linear programming (ILP)

What do we want to track? We will perhaps cheat a little, and enlarge typestate
to include integer range restrictions, perhaps even rational value range
restrictions. We will probably think in terms of sets of constraints expressed
as integer linear programming (ILP) tasks. The details of solving these will be
worked out in the project, but one can see that having a uniform concept of the
solution engine can be very helpful, not least to limit what we ask of it.

We have earlier highlighted the importance of knowing the validity of an object
reference. *Patinon* also needs to track member accesses. To call a getter, the
object must be in a state in which that member has been initialized. To call a
function that writes a member value, These are not difficult, at least on the
surface: the contracts for these functions require that the object be
initialized, and also mutable as appropriate.

If the graph of states is not trivial, then we need to deal with typestate
inequalities. A proper discussion of this is substantial, and we will expand on
preliminary documents as part of the *Patinon* project. In summary, *Patinon*
can deal with planar directed acyclic graphs, and can analyse state inequalities
by means of a dual ordering. If one creates a particular topological sort from
left to right, and another from right to left, then descendant (or conversely
antecedent) states will be on the same side of the two orderings.

### Some examples

*Example used in bottom-up (in brace syntax). Very sketchy.*

```
   s := 0
   loop i through [0 .. x.size()]:
      s += x[i]
   /
```

Let us make some rough observations about this, focusing on the array `x`.
Within the loop only read access is required. The forward analysis of typestate
would be less constraining. Thus the first pass of analysing the typestate would
only require that the array be initialized. However, shrink-wrapping would use
the fact that the read access is permissible provided that the array is
value-mutable. Consequently, its size is constant.

The contract for `x[i]` requires that `i` be greater than, or equal to, 0 and
less than the size. Thus we have a requirement contract that we express as two
linear programming constraints, with `i` and `x.size()` as the variables. In
this example the loop invariants established as above are exactly the same.

*Patinon* does not try to *match* what is ensured with what is required.
Instead, it looks to see if there are any solutions in the variables, here `i`
and `x.size()`, that are within what is ensured and outside of what is required.

From a pure correctness standpoing, *Patinon* does not need to compare terms in
its ILP tasks. However, for efficiency and diagnostics reasons it does need to
identify and process pairwise terms. This means, for instance, that *Patinon*
will quickly find that `i < x.size()` and `i >= x.size()` cannot be
simultaneously satisfied. The reason for doing this is that typestate is tracked
as sum-of-product constraints. This is typically convenient and compact.
However, the contract that is required by the access `x[i]` is negated, since we
need to find violating solutions. Thus the contract becomes a product of sums.
Such inverted constraints typically have many more terms when expanded out.
However, many terms in the sums can be eliminated by pairwise checking.

In order to have good diagnostic communication, *Patinon* needs to communicate
in meaningful ways what is violated and what the prevailing ensured constraints
are. Pairwise checks help with this, but they need to be combined with
interpretation information. Constraints can be seen as hierarchical. For
instance, `i >= 0` and `i < x.size()` are grouped together as arising from the
range of the induction variable. This information needs to be preserved and used
meaningfully.

Suppose that the programmer included a line of code that required that the array
`x` have mutable structure. While it is not realistic that such an error would
be typically be made for a simple example, one can easily imagine a more complex
situation in which a typestate constraint takes an unexpected form. Nonetheless,
even in this simple case, how would the error be surfaced to the user? It would
likely be associated with the fact that the constraints on `i` do not satisfy
the contract for `x[i]`, which is not what the programmer would see as the
error. How can hierarchical information be explorable in such a way that the
initial hint can begin a path to understanding? This is a task for *Patinon*.
(There are some fairly straightforward possibilities for this example.)
