# implement a red black tree
# https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
# https://www.cs.usfca.edu/~galles/visualization/RedBlack.html

class Node:
    def __init__(self, key, color, parent, left, right):
        self.key = key
        self.color = color
        self.parent = parent
        self.left = left
        self.right = right
    
    def __str__(self):
        return str(self.key)

class RedBlackTree:
    def __init__(self):
        self.nil = Node(None, 'black', None, None, None)
        self.root = self.nil
    
    def insert(self, key):
        node = Node(key, 'red', None, self.nil, self.nil)
        self.insert_node(node)
    
    def insert_node(self, node):
        parent = self.nil
        current = self.root
        while current != self.nil:
            parent = current
            if node.key < current.key:
                current = current.left
            else:
                current = current.right
        node.parent = parent
        if parent == self.nil:
            self.root = node
        elif node.key < parent.key:
            parent.left = node
        else:
            parent.right = node
        node.left = self.nil
        node.right = self.nil
        node.color = 'red'
        self.insert_fixup(node)
    
    def insert_fixup(self, node):
        while node.parent.color == 'red':
            if node.parent == node.parent.parent.left:
                y = node.parent.parent.right
                if y.color == 'red':
                    node.parent.color = 'black'
                    y.color = 'black'
                    node.parent.parent.color = 'red'
                    node = node.parent.parent
                else:
                    if node == node.parent.right:
                        node = node.parent
                        self.left_rotate(node)
                    node.parent.color = 'black'
                    node.parent.parent.color = 'red'
                    self.right_rotate(node.parent.parent)
            else:
                y = node.parent.parent.left
                if y.color == 'red':
                    node.parent.color = 'black'
                    y.color = 'black'
                    node.parent.parent.color = 'red'
                    node = node.parent.parent
                else:
                    if node == node.parent.left:
                        node = node.parent
                        self.right_rotate(node)
                    node.parent.color = 'black'
                    node.parent.parent.color = 'red'
                    self.left_rotate(node.parent.parent)
        self.root.color = 'black'
    
    def left_rotate(self, node):
        y = node.right
        node.right = y.left
        if y.left != self.nil:
            y.left.parent = node
        y.parent = node.parent
        if node.parent == self.nil:
            self.root = y
        elif node == node.parent.left:
            node.parent.left = y
        else:
            node.parent.right = y
        y.left = node
        node.parent = y
    
    def right_rotate(self, node):
        y = node.left
        node.left = y.right
        if y.right != self.nil:
            y.right.parent = node
        y.parent = node.parent
        if node.parent == self.nil:
            self.root = y
        elif node == node.parent.right:
            node.parent.right = y
        else:
            node.parent.left = y
        y.right = node
        node.parent = y
    
    def print_tree(self):
        self.print_node(self.root)
    
    def print_node(self, node):
        if node == self.nil:
            return
        self.print_node(node.left)
        print(node.key, node.color)
        self.print_node(node.right)
    
    def search(self, key):
        return self.search_node(self.root, key)
    
    def search_node(self, node, key):
        if node == self.nil or key == node.key:
            return node
        if key < node.key:
            return self.search_node(node.left, key)
        else:
            return self.search_node(node.right, key)
    
    def delete(self, key):
        node = self.search(key)
        if node == self.nil:
            return
        self.delete_node(node)
    
    def delete_node(self, node):
        y = node
        y_original_color = y.color
        if node.left == self.nil:
            x = node.right
            self.transplant(node, node.right)
        elif node.right == self.nil:
            x = node.left
            self.transplant(node, node.left)
        else:
            y = self.minimum(node.right)
            y_original_color = y.color
            x = y.right
            if y.parent == node:
                x.parent = y
            else:
                self.transplant(y, y.right)
                y.right = node.right
                y.right.parent = y
            self.transplant(node, y)
            y.left = node.left
            y.left.parent = y
            y.color = node.color
        if y_original_color == 'black':
            self.delete_fixup(x)
    
    def transplant(self, u, v):
        if u.parent == self.nil:
            self.root = v
        elif u == u.parent.left:
            u.parent.left = v
        else:
            u.parent.right = v
        v.parent = u.parent
    
    def minimum(self, node):
        while node.left != self.nil:
            node = node.left
        return node
    
    def delete_fixup(self, node):
        while node != self.root and node.color == 'black':
            if node == node.parent.left:
                w = node.parent.right
                if w.color == 'red':
                    w.color = 'black'
                    node.parent.color = 'red'
                    self.left_rotate(node.parent)
                    w = node.parent.right
                if w.left.color == 'black' and w.right.color == 'black':
                    w.color = 'red'
                    node = node.parent
                else:
                    if w.right.color == 'black':
                        w.left.color = 'black'
                        w.color = 'red'
                        self.right_rotate(w)
                        w = node.parent.right
                    w.color = node.parent.color
                    node.parent.color = 'black'
                    w.right.color = 'black'
                    self.left_rotate(node.parent)
                    node = self.root
            else:
                w = node.parent.left
                if w.color == 'red':
                    w.color = 'black'
                    node.parent.color = 'red'
                    self.right_rotate(node.parent)
                    w = node.parent.left
                if w.right.color == 'black' and w.left.color == 'black':
                    w.color = 'red'
                    node = node.parent
                else:
                    if w.left.color == 'black':
                        w.right.color = 'black'
                        w.color = 'red'
                        self.left_rotate(w)
                        w = node.parent.left
                    w.color = node.parent.color
                    node.parent.color = 'black'
                    w.left.color = 'black'
                    self.right_rotate(node.parent)
                    node = self.root
        node.color = 'black'
    
    def successor(self, node):
        if node.right != self.nil:
            return self.minimum(node.right)
        y = node.parent
        while y != self.nil and node == y.right:
            node = y
            y = y.parent
        return y
    
    def predecessor(self, node):
        if node.left != self.nil:
            return self.maximum(node.left)
        y = node.parent
        while y != self.nil and node == y.left:
            node = y
            y = y.parent
        return y
    
    def maximum(self, node):
        while node.right != self.nil:
            node = node.right
        return node
    
    def inorder_walk(self):
        self.inorder_walk_node(self.root)
    
    def inorder_walk_node(self, node):
        if node == self.nil:
            return
        self.inorder_walk_node(node.left)
        print(node.key)
        self.inorder_walk_node(node.right)
    
    def preorder_walk(self):
        self.preorder_walk_node(self.root)
    
    def preorder_walk_node(self, node):
        if node == self.nil:
            return
        print(node.key)
        self.preorder_walk_node(node.left)
        self.preorder_walk_node(node.right)
    
    def postorder_walk(self):
        self.postorder_walk_node(self.root)
    
    def postorder_walk_node(self, node):
        if node == self.nil:
            return
        self.postorder_walk_node(node.left)
        self.postorder_walk_node(node.right)
        print(node.key)
    
    def height(self):
        return self.height_node(self.root)
    
    def height_node(self, node):
        if node == self.nil:
            return 0
        return 1 + max(self.height_node(node.left), self.height_node(node.right))
    
    def size(self):
        return self.size_node(self.root)
    
    def size_node(self, node):
        if node == self.nil:
            return 0
        return 1 + self.size_node(node.left) + self.size_node(node.right)
    


if __name__ == '__main__':

    # 1. Create a tree
    tree = RedBlackTree()

    # 2. Insert some nodes
    tree.insert(11)
    tree.insert(2)
    tree.insert(14)
    tree.insert(1)
    tree.insert(7)
    tree.insert(15)
    tree.insert(5)
    tree.insert(8)
    tree.insert(4)
    
    # 3. Print the tree
    tree.inorder_walk()



    
