class Point {

    // Private fields
    int x;
    int y;

    // Constructor
    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    // Method
    public Point add(Point other) {
        return new Point(this.x + other.x, this.y + other.y);
    }

}
