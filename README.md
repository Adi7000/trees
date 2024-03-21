# Trees, Trees and More Trees

## Major Innovations
None outlined

## Rationale

1. What does a red-black tree provide that cannot be accomplished with ordinary binary search trees?
   - Red-black trees provide a more balanced structure and guarantees worst-case searching, insertion and deletion of `O(logn)`
     as opposed to `O(n)` for binary-trees
2. Do you need to apply any kind of error handling in your system?
   - Yes, Option is well used in our program to ensure that we are not using None
3. What components do the Red-black tree and AVL tree have in common?
   - Both trees share the same structure with the Red-black tree only having an extra color field
   - Part of the key insertion algorithm is shared between the both trees where the binary tree insertion is performed
   - The left and right rotation methods are also shared between the two trees
   - Some smaller methods shared between the trees are the height, empty, leaf count, inorder traversal, print
4. How do we construct our design to “allow it to be efficiently and effectively extended”?
   - The `trees.rs` file contains structure for a binary node's representation. This representation can be extended to any other type of binary tree (Avl, RedBlack, etc) by simply informing the tree.rs file of the extended design's existance and handling very niche cases.
   - By having the majority of the binary tree functions exist in `tree.rs` and having the Red-black tree or AVL tree call these methods, we ensure that any extensions to this design can take full advantage of the pre existing functions mentioned above.
   - For non-ninary trees (such as 2-3-4 trees), the representation of a node could be extended by defining another node type in `tree::Node` and then providing appropriate additonal information in a new file specific to the new tree type

## Errors, Faults, Defects, Missing Functionality

- As the tree gets bigger, the printing correctness degrades resulting in harder to read graphs.
- Issues deleting when there is more than 11 nodes in Red-black trees.
  - Issue more specifically happens when the parent of the deleted node (i.e. the inorder successor of the right subtree of the input key node) is red. But this condition (parent being red) is only necessary (but not sufficient) for the incorrect rebalancing to take place.


## User Manual

Example of Functionality

1. Start the program with `cargo run example`
2. Watch the preloaded example do some common operations

Manual Input Through CLI

1. Start the program with `cargo run manual`
2. Follow the prompts to use either AVL or Red-Black tree
3. Follow the prompts to insert nodes, delete nodes, or print the tree

## Discussion on benchmark results
- Please open `criterion/report/index.html` in your browser to view the benchmark graphs and results
- Here each link shows the average time it takes to insert or search `n` number of elements (Sampled over 100 attempts)
- For insertions, the AVL tree was approximately twice as fast as the RedBlack tree 
  - For example, when inserting 130 000 elements, the AVL tree took 24.672 ms and the Red Black tree took 51.184 ms
  - This trend was the same for other insertion counts aswell so the AVL tree has better worst case performance for insertions
- For searches, the AVL tree took marginaly longer than the Red Black tree
  - For example, when searching 13 000 elements, the AVL tree took 645.99 µs and the Red Black tree took 607.35 µs
  - This trend was also consistant for different number of searches
- Overall, the AVL tree is the better choice due to its superior performance (2x faster) with inserting. For most tasks, the marginally faster searches do not justify using a red black tree especially because the cost is much more greatly affected by 
insertion times. The only use case of RedBlack would be highly search intensive jobs.
- Deletion test cases should also be accomodated for benchmarking as it is an important measure of performnace for many tasks
- Additionally, benchmarking against other industry leading data structures would be a good idea to show a true comparisson with
what is available. This would help customers better decide the appropriate data structure better for the needs of their task. For example, a user may choose to use an unordered set for constant time lookups if lookups are the majority of the demand from their system

## Marketing Video
https://www.youtube.com/watch?v=JGV3mn_0VIk&t=3s
