% Glossary
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


# Glossary

## A-F

Adaptive planning: {#adaptive}
:   Open source projects tend to be light on planning, and adapt as they
    progress. Furthermore, adaptive planning is considered to be a key aspect of
    *agile* programming practices.

Bike-shedding, or the *Law of triviality*: {#bikeshed}
:   When properly used, this refers to the phenomenon that disproportionate
    weight is often given to relatively unimportant issues. It is often used
    incorrectly to suggest that the issues are unimportant in an absolute sense.

:   Reference:
    <a name="LawOfTrivialityWiki"></a>[Law of triviality](https://en.wikipedia.org/wiki/Law_of_triviality),
    in *Wikipedia: The Free Encyclopedia*, retrieved 2019-08-24.

Data-flow analysis: {#dataflow}
:   Patinon borrows much from the ideas of data-flow analysis, even though we
    normally speak in terms of [typestate](#typestate) analysis. The most
    fundamental contribution of data-flow analysis is to work out the exact
    patterns of data definition and use. For example, a variable might be
    initialized differently along each branch. A compiler should detect a bug if
    we fail to initialize the variable. Sadly, RAII and the conceptual approach
    to design that it encourages, can defeat this analysis, since data should be
    "initialized" before the intentional initialization.

:   Reference:
    <a name="DataflowWiki"></a>[Data-flow analysis](https://en.wikipedia.org/wiki/Data-flow_analysis),
    in *Wikipedia: The Free Encyclopedia*, retrieved 2019-07-16.

Frequent releases: {#frequent}
:   We use this term to encompass approaches to software development under which
    releases are made frequently. In the past releases would in most cases be
    cut infrequently with *beta* test versions, and so on. In contrast, many OSS
    projects adopted shorter release cycles. With the growth of the internet,
    successful companies would release a new live version more or less every
    week. After this *agile* programming practices adopted a philosophy of
    adding features progressively to a functioning system. All this resulted in
    a fundamental shift in attitude and approach. A system should work before
    and after every committed change. While bug fixes are inevitable, the
    emphasis is that each change can be tested productively before it is
    submitted.

Fluid contributions: {#adhoc}
:   We use this term to denote the development patterns and needs of projects
    with a large and changing set of contributors. This is common in large
    software organizations and on OSS projects. Part of this is that developers
    join and leave teams, and so onboarding and transfer of knowledge is
    important. But actually the greater challenge comes from the fact that many
    contributors make *guest appearances*, adding one or two features, writing a
    few changes with tests. While the reviewer may be a core long-term team
    member, many contributors need to be able to learn quickly about a part of
    the system and be able to change that part with confidence that tests
    provide a strong *fence around the playground*.

Fluid development: {#fluid}
:   With Patinon we use *fluid development* to encompass various aspects, often
    challenging, of software development that is changeable and that flows over
    time. (As opposed, say, to being structured around major scheduled
    milestones.) Specifically we include fluid contributions, fluid release
    schedules and fluid planning. (See the other entries herein for
    [contributions](#adhoc), [release schedules](#frequent) and
    [planning](#adaptive).)

## G-L

Integer linear programming (ILP) {#ilp}
:   Integer linear programming (ILP) problems are like linear programming
    problems, with the variables restricted to integers. While the solution of
    ILP problems are NP-hard, Patinon on needs to test for the existence of a
    solution. In many cases one can prove that no solution exists even if the
    variables are relaxed to real numbers. In other cases the size of the
    problem is small. In the long run there might be a security consideration,
    so a production successor to Patinon would have to ensure that solvers
    terminate within a predetermined limit on effort.

:   Patinon will probably need to deal with ratio variables.

:   The main difficulty with quick non-existence checking for solutions will be
    in the fact that Patinon will need constraints specified in terms of
    inequalities and equalities. Also these constraints will need to be negated.
    A possible approach will be to add a moat variable such as $$e$$. Each
    negated bound could be separated by this (potentially very small) amount
    from the unnegated bound. If there is a solution to the integer problem,
    there must be a solution to the moat-adjusted real linear programming
    problem with non-zero moat.

:   On the other hand, if ratio ILP problems can be converted into pure ILP
    problems, in the non-mixed case, then perhaps we can require that there be a
    solution with $$e=1$$.

:   Some ILP tasks in Patinon may have variable that are not discrete, and so it
    needs to handle the mixed generalization

:   Reference
    <a name="IlpWiki"></a>[Integer programming](https://en.wikipedia.org/wiki/Integer_programming),
    in *Wikipedia: The Free Encyclopedia*, retrieved 2019-07-22.

## M-R

## S-Z

st-planar graph (#stplanar)
:   Reference
    <a name="StplanarWiki"></a>[st-planar graph](https://en.wikipedia.org/wiki/St-planar_graph),
    in *Wikipedia: The Free Encyclopedia*, retrieved 2019-07-22.

Typestate: {#typestate}
:   Typestate is state information that augments a data type. Typestates are
    states like *unallocated*, *initialized*, *error* and *configured*. Only
    specific transitions are permitted, and these are given a direction
    (commonly towards the more capable state). The typestate graph must be a
    DAG, and is most often a tree or just linear chain. Function signatures and
    return values are augmented with typestate, and these restrictions form the
    bulk of contract specification. Function signatures must match in the usual
    way: typestate information is not used to resolve the choice of function to
    call. In a subsequent stage of code analysis, later during transformation
    ("compile") time, typestate is tracked. Most often input arguments are
    required to be valid and initialized. Functions most often provide return
    values that are valid and initialized.

:   Typestate graphs should be simple. Typestates are *not* intended to
    micro-manage an object. Rather, they provide assurances so that functions do
    not have to check the readiness of input data. In turn, functions make basic
    promises about data on exit.

:   See also [data-flow analysis](#dataflow).

:   Reference: [Typestate analysis](supporting.md#TypestateWiki)
