# Circle Tree
Creates a random tree where each node can have `(2..n)` children. Based on this tree, line segments are created that each split the space recursively so that `n` circles could be drawn in each segment, based on optimal circle packing. Then these line segments are used to cast a ray and find the intersection point with previously added segments, to create the final look.
