class Point(object):

    # Constructor
    def __init__(self, x, y):
        self.x = x
        self.y = y

    # Method
    def add(self, other):
        return Point(self.x + other.x, self.y + other.y)
