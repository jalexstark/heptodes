% Isolation
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


# Isolation

### Rust

One of this innovations of *Rust* is in providing a fully-developed language
that controls access to data. (A major achievement can sometimes be summarized
in few words.) *Patinon* plans to build on that.

#### Mutability rules

*Rust* allows a data object to be referenced in one of two ways. Either

*   the object is non-mutable, in which case there may be multiple paths to
    reach it, or
*   the object is mutable, in which case there may only be one path to reach to
    it, that is it is exclusive.

Aside: We will need to figure out what traits are unwound when leaving scope,
and those which continue on as execution leaves some nesting.

#### Lifetimes

*Rust* works to ensure that a piece of data cannot be accessed after it is
freed. Since this means making sure that any parent object is in scope, the
framework cannot be trivial. Consider the following example.

```
struct Point ⌈
   x :int
   y :int
⌋
```

Suppose that we create an instance and instance, `p`, of `Point`. Then suppose
that we create a reference `r` to `p.x`. If `p` goes out of scope, and we
destroy it, before we remove the reference `r`, we have a problem. If this were
all we were to do, we could maintain a record of the dependent lifetimes.
However, what if the member field `x` is not exposed, but instead we provide a
function `GetX()`?

This could get even more involved with functions that are not straight getters,
and still return data that refers to dependent parts of input data. *Rust* deals
with these needs through lifetimes. Lifetimes are a kind of trait that variables
possess. Methods and functions specify (with sensible defaults) which of their
input lifetimes should be associated with each returned reference.

### Isolation

The concept of `isolated` data was set out in
[GordonCS12](supporting.md#GordonCS12) (and articles referenced therein, but not
accessible). It starts with two categories of data.

*   *Immutable* data is not only non-mutable. It has the guarantee that the data
    can only be reached through immutable paths. Users of the data can be sure
    that on one else will change it.
*   Mutable data is exclusively owned. That is, mutable data that is reachable
    from an isolated root can only be reached through that root.

I am not clever enough to understand how to establish isolation automatically.
Working through how it might be done with less sophistication, I ended up with
something a bit like *Rust*. Let us now think how the two might be brought
together.

Really *isolated* is a qualification of a local variable or member field saying
that the member's compass is itself.

In regards to the aforementioned article, we might not manage all of the
"recovery" of isolation, but might manage to define a clear subset of "framing".

#### Isolation table

More vague than it looks, and probably incorrect.

Permit references from this / Guarantee other references to this / Guarantee
other references to reachable from this.

Trait     | Permit from | Guarantee to | Guarantee reachable
--------- | ----------- | ------------ | -------------------
readable  | R           | W            | W
writable  | W           | W            | W
immutable | R           | R            | Path-immutable
isolated  | W           | No           | Path-immutable
unique    | W           | None         | None

#### Loosening isolation, widening compass

The isolation of an object should be capable of being widened. This capability,
and the limits on it, will be require careful definition. For example, the
compass of a value field should not be widened, beyond the owning object's
compass. That object's compass could be widened first.

### Rust and isolation compared

One can view the two cases that Rust permits as special cases of isolation.
Either (a) an isolated object only has parts that are not mutable and exclusive
parts, in which case it is immutable, or (b) an isolated object has no immutable
parts and so all its content is mutable and therefore exclusive.

On the other hand, Rust has fleshed out some foundational traits (and control
thereof) that aid simplicity in *Patinon*. The essential need is to avoid
anything especially clever in regard to establishing whether objects are
correctly isolated. As mentioned above, we think that isolation of objects can
be be wrestled into something manageable. We think that can be achieved by
introducing a concept: *compass*.

Note: As we mentioned previously, we can think through use cases involving
sub-objects. Someone might object that these are just field shadows. However,
object methods might do more than `GetX()`. So the problems we want to resolve
are not simply those of making copies of references.

Isolated objects are allowed internal cross references. So I might obtain a
reference to one part and copy it (or move it perhaps) to another. But I can
only do this if I know that both source and destination are part of the same
isolation. Further, I can only create a reach path to a mutable part from
another mutable part. We call this tentatively the *compass*. This alone would
cause us to reinvent a mechanism like *Rust's* lifetimes. And we are going to
need those, too.

These ideas are not trivial, but it should be emphasized that they would not be
needed in introductory programs. There are gradual and measurable benefits from
all this as programs become more complicated. For example, associating compass
with objects enables us to provide straightforward rules for widening and
narrowing the compass of isolation. For example, it is mostly a matter of course
to move a self-isolated sub-field of an object to another. But if we want to
create a second temporary object we need to widen the compass to a higher
object, drop our access to the original, permitting narrowing the isolation down
to the new for temporary work. This may sound a little complicated, but it
enables us to control access in subtle ways when that is necessary.

### A note

Using field values as primary would be challenging. Idea: `create primary(..)`
would demand lifetime (ownership), uniqueness, and return a primary reference
that could be checked by sanitizers.

### Internal references

*Patinon* will distinguish between primary and secondary references, and
probably not directly support weak references. This enables data structures to
maintain a set of objects in, say, a hash map, and then use them elsewhere in
the same data structure. Another classic example is a doubly-linked list. The
forward references would probably be chosen to be primary, the reverse to be
secondary.

*   There can only be one primary reference, but can, subject to isolation, have
    multiple secondary references.
*   The primary reference is a kind of ownership. Basically, ask which reference
    would be used during destruction? Which part of the overarching data
    structure would be traversed to clear up owned heap objects?
*   Stack data, or references created to an internal (sub) object would treat
    those as owned values and therefore no primary reference would be needed. In
    other words, if secondary reference do not refer to a directly-allocated
    heap object but instead to a part or to a local variable, then there would
    be no corresponding primary reference.
*   This is not a complete solution to dangling references. It does not
    guarantee that secondary references are all removed before destruction.
    Lifetimes are required for that.

All immutable references are treated as secondary, not owned. One starts with a
mutable object, or primary reference to that object. Once the object is made
immutable ready for immutable reach duplication, the ownership might not be
transferable. This may need one extra rule: we need to prevent removal of the
primary reference. In practice this means that a pliant reference to an
immutable object may have to lose it pliancy.

Rule: Once all the primary mutable referenced objects within an isolation have
been freed, all the secondary references within that isolation are reassignable.

Definition: a *nullable* reference is one that can be read and compared with
null. (More correctly, one can test it with `!!`.) A reference without this
trait can be assigned null (the assignment does not actually happen), so that
*Patinon* can track that it is invalid. For a nullable reference, the assignment
actually happens.

*   When moving a reference away from a nullable reference, it is assigned null.

Note: We should probably have `nullify r` instead of assignment, to avoid
confusion. One can think of the nullability as a property of a container-like
wrapper around a raw reference.

Note continued: Or it could be a pair of typestates on all references. The main
typestate would be free - alloc - init - non-null-init, and a reference that is
not nullable would kave to skip the ordinary init state when passed.

Note: *Patinon* is not safe in regard to references. It does not ensure that all
of an object's secondary references are invalidated before it is destroyed.
However, it does ensure that all secondary references within an object (members
thereof) are invalidated before permitting the related typestate transition.
This means that the programmer has to be intentional.

*   It would probably be too demanding and restrictive to require that, when
    invalidating a secondary reference, the "code" has knowledge of the
    continued existence of the primary reference. This likely to be unworkable,
    not least because one could only check the correspondence of type. That
    said, may be a "rigorous mode" could be provided in which the match of the
    two references could be checked with a runtime error. Possibly useful for
    sanitizer tests. While we would not want to create confusion, it might be
    that some sort of move-nullify semantics could be provided, that is not just
    assigning null to the secondary reference.

#### Other rules

*   Immutable references are never deleted. This implies that references cannot
    logically be both primary and immutable. There must be a mutable path to any
    primary, and the mutability (isolation) must have been recovered before its
    destruction.

#### Relinquishing

Note: More like is-a-copy vs copyable vs exclusive (really sole path to reach).

Actually, secondary references could be relinquished back to another secondary
reference. After all, a secondary reference might be made from a secondary
reference.

This also helps with the case of immutable references. When a reference or an
object is cast to immutable, its primary references become secondary until
re-scoped.

#### Exclusivity

Exclusivity basically means that an object, local or referenced to heap, that is
isolated, has never been used for a secondary reference, so that it can only be
reached one way. Immutable references within are OK, so this is a slightly
looser definition of exclusivity.

It is therefore fine to convert the isolated reference to immutable, then make
secondary references. Once all secondary references are relinquished, the
immutability of the original isolated object can be dropped.

Thus it is illegal to free an object while it is immutable.

### Combined compasses, keyed compasses, partitioning of compasses

Note: At simplest all compasses could be `wrangler.Get(key)` and when in
content-only access state, or persistent state. Returned references guaranteed
to be disjoint on distinct keys. Also, one could make up fake return values
(objects) for non-object compasses. By "guaranteed" it must be "proved" that no
part of one can be reached from another.

Keyed compasses are those, that based on a unique value of a key in an object,
are unique. Functions that return references with such a compass are guaranteed
to be isolated within it, and therefore mutable data is guaranteed to be
disjoint. The big benefit is that compasses can be narrowed.

*Patinon* may well support combined (AND-logic) of lifetimes. This is so that
one can usefully deal with widely-encompassing compasses. The key need, and the
key logic required, will be so that it is easier to narrow down separately
isolated objects when the compass goes out of scope.

This sounds a little paradoxical. Suppose that one is making extra mutable
references, say, so that some collection of data is getting divided up into 4
separate jobs so that they can be handled by 4 threads. To copy the references,
we need to create a shared compass. We need to know that the source reference is
itself, relative to other references that we might use, is isolated. It could
itself be isolated, or it could be relatively isolated via a keyed compass.

This also helps to recover the compass later when restoring scope.

But even simple trees and reference getters can lead to a need for
specialization of compasses. Consider the following.

```
struct Point ⌈
   x :int
   y :int
⌋
struct Rectangle ⌈
   top_left :isolated :Point
   bottom_right :isolated :Point

   <Methods...>
⌋
```

Now, with a bit of imagination but stretching the plausibility of the example,
we might add a couple of methods to Rectangle

```
   GetLeft(self) --> left :&int ⌈
      left := &self.top_left.x
   ⌋
   GetBottom(self) --> bottom :&int ⌈
      bottom := &self.bottom_right.y
   ⌋
```

Suppose we have an isolated Rectangle object `rect`, such as a local variable.
Now suppose that we get references to the left and the bottom using these
methods. The reference to the left integer is isolated with respect to the
`top_left` field of the Rectangle. But we could have made cross references to
`top_left` within `rect`, since it is isolated, so the isolation of `left` is
not quite guaranteed. (We are assured that no copies of references to content
would be made.) However, if we drop `rect` (eliminate the reach path
temporarily) we automatically narrow the isolation of `left` to `top_left`. To
make any copies, any new object containing them would have to have the isolation
and lifetime of `top_left`.

Suppose we drop the reach path to `rect`, create a new isolated object, and make
a couple of reference coies to `left` within it. This is fine. However, if we
un-drop `rect` while that new object is reachable, we have a problem, because it
would result in two reach paths.

Obviously this can get a bit complicated. In general, however reference getters
are not typically going to return compasses or lifetimes that are different from
their parent objects.

This sub-compass need, such as the isolation of `top_left` within `Rect` will
probably be handled as a chain of compasses, that is something like `(Rect,
top_left)`. As variables, shadows, and so on are added within scoped data, the
result will become a graph of some sort.

#### Keyed compasss partitions

Even memory is a bit like keyed partitioning. Unique memory pointers are part of
the guarantees of isolation.

Idea: Partitioning of compasses according to mutually unique keys.

Difficulty: Does not quite handle the circular (double) linked list case. There
we want a partitioning in which some references span the sub-compasses.

#### Unfolding of partitioning

Two approaches

1.  The more managed approach. State that a compass should now be replaced by a
    set of compasses with non-overlapping access, partitioning the compass. Not
    necessarily creating mutually-isolated sub-compasses in all scenarios.

2.  Create a set of sub-compasses, and then move the overarching compass out of
    scope. At that point, the partitioning would be inferred.

#### Keys as a generalization of references

One can think of the compass of a simply isolated object as being a keyed
compass where the reference is the key. *Patinon* will most likely have a real
"conversion" between reference-as-pointer and reference-as-key, to make this
interpretation explicit. Understood in this way the reference-as-key is a
special case. Keying of compasses is not a clever extension of references. For
example, net and previous nodes could be "keys", and nodes accessed via wrangler
and its compasses.

The generalization has another consequence: we should consider making the
primary vs secondary keying protection system just that. What we develop for
references we could do for keys, including the idea of associating the
destruction of a set of objects with the dropping of corresponding keys.

An example of this might be use of a set of unique string names for the nodes
and edges in a graph. The nodes and edges might be managed via two hash maps
keyed by the strings. The graph connectivity would be described in terms of
secondary uses of the keys.

If, when an instance of a data type is created, the *primary* trait is
associated with it, then copies would have to follow the rules for secondary
object management.

#### Controlling deletion

*Patinon* does not track the safety of deletion of primary objects directly.
Instead, as eluded to above, every set of references has primary and secondary
copies. When the application shuts down, or the encompassing object is deleted,
all primary references would need to have been deleted, as would all secondary
references. Thus the trick is to associate the deletion of every secondary
reference with its primary reference.

This process, as outlined above, of giving secondary references back to known
primary references, may not be programmatically burdensome. We will learn about
that with experience. *Patinon* needs to have good capabilities for eliding work
that is unnecessary when in non-debug execution. One could even have an `elide`
trait that checks that data-creation elision works as expected.

It *may* be possible to develop some system for a degree of checking
automatically, by retaining knowledge of the association of references, but this
would be considered advanced experimentation.

#### Examples of partitioning

*   Traversal of an execution graph. Parts of such graphs can be executed
    simultaneously because input data is ready and output data is disjoint. The
    input (edge) data is immutable, the output (edge) data is mutable and
    isolated. While *Patinon* will probably not check all of this for the
    programmer, the code may need primarily to ensure that the keys to edge data
    are appropriately disjoint. Then *Patinon* will ensure the correct isolation
    partitioning.
*   Divide-and-conquer of an image by tiling. Ranges created by each tile in the
    output would be distinct, and so sub-arrays would be guaranteed to be
    isolated. The ranges make the compass partitioning keys.
*   In something like an FFT, stages take interspersed data. A simple example of
    this is one job taking the even-indexed elements of a vector, and another
    taking the odd-indexed elements. The sub-array specifications for these
    would be considered distinct keys. One can think of this as reshaping the
    data into an array, and selecting distinct columns.

#### Random extra notes

*   Idea might be that there is a safe deposit box that will give you special
    access to partitioned sub-objects if you have unique key.
*   Maybe would be easiest to have a "concierge" typestate in which there are
    limited getter methods and immutables such as size of array. Indeed it
    probably would be necessary to put the main object into the fixed-size
    typestate even if a specific concierge state proved not necessary. Only the
    content being divided up between jobs (sub-objects) should be mutable.
*   We need to be clear that immutable and non-mutable are different. A
    non-mutable object can be treated externally as immutable, which would be an
    explicit choice. A non-mutable object would have to obey the rules of
    isolation internally.

#### A general concept

In *Patinon* one of two scenarios applies to the lifetime of an object.

1.  The object is created by its owner, managed in regards to its compass, and
    destroyed by that owner. Secondary reach paths may be created in the course
    of its life. It may be lent to another object or function for a time.
2.  An object may be created and transferred in entirety to another owner. A
    simple example would be passing a string to a logging function.

These are not meant to be mixed. One is not meant to create secondary reach
paths and pass them to a functions that thinks it has ownership.

#### Assignment of references

The assignment of a references must adhere to the compass rules. Any of the
following can apply.

1.  It has no compass, because it is immutable.
2.  It is isolated in its own right and so can be moved.
3.  It shares compass with the destination. The move is within a common compass.
4.  Its source compass is a descendant of the destination compass.

In order to do some moves, a compass might have to be expanded to a parent
compass.

#### Exclusive partitioning

If a compass is partitioned into keyed exclusive sub-compasses, then the union
of the sub-compasses that is provided to a function or operation forms a new
(sub) compass that must be enclose everything in any move or copy.

Also need way to say if a field is exclusive to a compass. For example, the
contents (reference) of the node of a linked list is exclusively by the node's
sub-compass of the list wrangler's compass.

#### Secondary reference rules

*Rule:* An *isolated* field cannot have secondary references made of it. (Note:
Why "of it"? Yes, maybe, "reach-to".) We would probably need an alternative
qualifier to say when we can take an object that is its own compass and create
secondary references. This does mean that, in some sense, there are multiple
paths to reach the contents. The preference is to make the "default" behaviour
conservative. The trait might just be (value) *primary*, indicating that
secondary references are allowed. The *primary* trait would also be useful
during deletion, since a sanitizer might be tracking secondary references and
would know the point of destruction, and therefore the point to check.

The difficulty with the above is when we get a descendant value within a primary
object, and where the descendant's value is not itself primary. We probably want
to allow this. We would be tracking the relationship between the two while we
have live (local) copies of the reach paths. However, we might not be able to
extend that tracking when a descedant's path is copied.

When we create a secondary reference to a descendant of an object, that creates
a tenant secondary reference. We need to drop all references to all parts, which
must all be within the same compass, before the lienholder is freed.

#### Assignment to descendants

When we move a reach path to a parent we might have to expand the compass. How
do we reverse this process?

It is not hard to move a reach path to a descendant. The greater difficulty is
narrowing compasses.

*   We may be able to leverage partitioning of compasses into keyed exlcusive
    compasses.
*   A sole-proprietor is guaranteed to be owned and otherwise compassed by
    itself. We might be able to leverage this.
*   All descendants in scope are tracked and associated with the owner.
    Therefore we can move the topmost ancestor of any remaining in scope.
*   If the landlord goes out of scope, then other paths are no longer reachable.
    This might be too temporary, though.

#### Disjoint partitioning and narrowing of compass

If a landlord goes out of scope, can we narrow the objects returned by getters
to tighter compasses?

If the returned objects are isolated with respect to the landlord, they can be
narrowed, but we may find this generally not to be the case. It certainly cannot
be the case for data that is only mutually distinct as a result of disjoint
getters, which return disjoint compasses.

Disjoint compasses are a fairly simple way to narrow compasses. It does
introduce complexity, but in a way that is gradual. The fact that we encounter
it in early obvious cases like hash maps and linked lists is actually
reassuring.

### The difficulty with doubly-linked lists

Doubly-linked lists are a pathological case. They may well not be handled
"safely" by *Patinon*. We say "pathological" because they are simple, and yet
not amenable to isolation. We expect that, ironically, most structures will be
manageable.

In circular lists one normally accesses by a node reference. However, the
compass is not that at all. There needs to be a list-wrangler object. We will
approach this by developing a scheme with an actual object and then see how it
works out to make all the references to it, and all its content, virtual.

The difficulty with doubly linked lists is that the backward links (we assume
here the secondary links) cannot easily be associated in a parameterized fashion
with the isolation of the primary forward link.

#### Fundamentals

The central realization wrt lists is that there is a list-wide compass. This can
be embodied in a list wrangler, which might be void and even virtual (true
sense, not C++). Even if the wrangler is virtual, *Patinon* may well require
that the code be written as if it is real and is instantiated. Sanitizers might
actually create instantions.

#### Containment

Linked lists could be of a value (as pass-through *derived* class), or isolated
reference, or primary reference or secondary reference.

The content itself should never link to the structure of the list. A node's
content should either be strictly isolated in itself, or references to and from
its contents should be to other node contents, or be otherwise external to the
list.

*   This means that a compass could be a nesting of allowed and disallowed
    cross-reference zones.
*   Rust has no field-level rights controls, and approach which avoids the
    complication but surely results in more `unsafe` code sections.
*   Class can themselves be isolated wrt a compass. Fields can be isolated,
    which would impose similar restrictions external to the field. Also, we
    should be able to annotate a field as otherwise disjoint from a compass. The
    object might be within the compass, and internally it would be isolated, but
    there would be no other compass benefits within that compass.

We should also add that in *Patinon* methods that return references are not
considered to be encapsulating. The internal storage is by the nature of a
reference accessor revealing part of the internal structure. That is not to say
that the class would not encapsulate and hide other details. A hash map does not
necessarily reveal its hashing approach. A reference counting container can be
externally immutable.

#### Exclusive partitioning

We considered more than one partitioning scheme, such as separate compasses for
cross links and separate compasses for node content. However, the simpler
approach seems best.

The simpler approach is to partition the wrangler compass by node reference key,
creating a sub-compass for each node. Operations on node connections need to
operate on the union of three compasses: the node in question, the forward and
backward nodes.

The union of sub-compasses is an incomplete compass. The rest of the compass
would have to be moved to a special frozen immutable-ish state, since it has
references to and from the union of active compasses.

#### More

Let us assume that all access to linked lists is through a pairing of a parent
list manager object and a node reference, something like `(manager, node)` as a
mini structure. In other words, if we ever try to get the next node from one
node, we do so with some idea of the list as a whole. If we get a node's
content, we might at that point get a sub-compass and a content reference,
largely dropping the actual path to the linked list.

It may not be necessary actually to pass the manager (wrangler) if the node has
the wrangler's compass associated with it. When elided fields are not elided
when sanitizing, there would be an actual secondary back referemce

So we could go as far as saying the references to list nodes could be hashed as
unique keys creating virtual sub-compasses for each node. Here the difficulty is
that a forward link would need to span the union of two keyed sub-compasses, and
the reverse link the span of the union of a different pair of keyed
sub-compasses. These links are the links *to* a node, perhaps one that we are
deleting.

This is an interesting task, because perhaps a temporary union of the three
compasses could be constructed, and then the compasses reduced to narrow the
isolations appropriately once a node has been deleted.

What is going on is that we need (a) hashed references as virtual records for
keyed compasses, and (b) somehow allowing something like a list node to have
isolations that span nodes in a chain, all ultimately "owned" by the list.

It might be necessary to treat the one-element list case differently from
others. The final code would be the same and simplified by the compiler. The
list wrangler can be destroyed when the list is empty because all secondary
references back to it would have been destroyed.

#### List example

> ```
> class CList{SomeType} ⌈
>    content :SomeType
>    next :&Clist{SomeType} :primary
>    prev :&Clist{SomeType} :secondary
>    manager :elided :CListWrangler{SomeType}
> ⌋
> class CListWrangler{SomeType} ⌈
>    ...
> ⌋
> ```

Whole list has a compass, and nodes have exclusively partitioned subcompasses.

A lot has to be handled. The trick is going to be to use typestate and compass
very efficiently and clearly.

### Destruction

While *Patinon* is not a garbage-collected language, one aspect of the design of
garbage-collected language is worth thinking about. This is that some objects
should not be collected because it is simply inappropriate, and many objects
should not be collected because their destructors want to do something that
should not happen in the background and unpredictably. An example of an object
that is not appropriate for collection would be a web server. One should bring a
server up and down under supervision. (Stated with the understanding that some
programmers disagree with this.)

*   Trivial destructors. Potentially containers of trivial objects.
*   Medium destructor. Test: is it OK to destroy as a member of a container?
    such as a vector of my object, or map or array. Destructors should be a
    little restricted in what they can do. The intention is that they will not
    be destroyed in the background, but they could be destroyed automatically at
    end of scope.
*   Complex destructors. These need properly managed state transitions. Going
    out of scope when not in unallocated typestate is erroneous.

This is not a universal taxonomy. For example, *Patinon* does not require that
file objects be unallocated when going out of scope, but does require that they
at least be closed.

### Replication, reaching and mutation

Extra needs.

### Shadows

*Patinon* allows shadows to exist within the current scope, while, for example,
*Rust* insists that mutable references are always moved. If one were to use the
other *Rust* rules, only slight change would be needed. Larger differences arise
as we extend the mutability framework to allow for isolated data.

Shadows can only be copied or referenced multiply within their compass.

### Compassed reference counter

Big benefit is thread safety without mutex.

### Immutability as a special club

*   Initiation into the club is that one has an exclusive reference and change
    it (within an inner scope, most likely) to not-mutable.
*   All data entering the club via this gate-check must necessarily only be
    reachable via an immutability path.
*   Once in the club, references can be copied as well as moved to new members.
*   Any isolated object can be converted freely to immutable, and thus initiated
    into the club, provided that all shadow references are amenable to being
    similarly converted.

Conclusion: We basically can convert (exclusive-isolated?) isolated to immutable
on request, but need to apply this to shadows.

#### Immutable references

New references to an immutable object are necessarily secondary. The primary
reference was only reachable when the object was mutable, and will become
primary again when mutability is restored.

### Unified proposal

Within a particular scope, at a specific point in the unfolding of program
execution, all reachable objects are isolated. Probably we could say that this
applies to all objects, except that we cannot say much about currently
unreachable ones.

This applies also to objects that cannot be reach, but that can reach our
reachable objects. That is, it includes the doctrine that our mutable data
cannot be reached by "someone else".
