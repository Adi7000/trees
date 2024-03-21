# Trees, Trees and More Trees

## Major Innovations

## Rationale

1. What does a red-black tree provide that cannot be accomplished with ordinary binary search trees?
   - Red-black trees provide a more balanced structure and guarantees worst-case searching, insertion and deletion of `O(logn)`
   as opposed to `O(n)` for binary-trees
2. Do you need to apply any kind of error handling in your system? (Yes)
   - Although not directly used, the panic macro is called in some places where the state of the system is not as expected
   For example, when rotating a node left, a node is assumed to have a right_child. If this assumption fails, the code will 
   panic with an error message
   - Additionally, Optional return values `Option<Rc<RefCell<TreeNode<T>>>>` are present for some of the functions in `tree.rs` to communicate that the state of the tree is bad and to prompt the calling tree to handle the problem. This bad state is handled by the `set_root_after_rotate()` function.
3. What components do the Red-black tree and AVL tree have in common?
   - Both trees share the same structure with the Red-black tree only having an extra color field
   - Part of the key insertion algorithm is shared between the both trees where the binary tree insertion is performed
   - The left and right rotation methods are also shared between the two trees
   - Some smaller methods shared between the trees are the height, empty, leaf count, inorder traversal, print
4. How do we construct our design to “allow it to be efficiently and effectively extended”?
   - The `tree.rs` file contains structure for a binary node's representation. This representation can be extended to any other type of binary tree (Avl, RedBlack, etc) by simply informing the tree.rs file of the extended design's existance and handling very niche cases. 
   - By having the majority of the binary tree functions exist in `tree.rs` and having the Red-black tree or AVL tree call these methods, we ensure that any extensions to this design can take full advantage of the pre existing functions mentioned above.
   - For non-ninary trees (such as 2-3-4 trees), the representation of a node could be extended by defining another node type in `tree::Node` and then providing appropriate additonal information in a new file specific to the new tree type

## Errors, Faults, Defects, Missing Functionality

- As the tree gets bigger, the printing correctness degrades resulting in harder to read graphs.
- Issues deleting when there is more than 11 nodes in Red-black trees.

## User Manual

## Marketing Video