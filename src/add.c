int add(int n1, int n2) {
    return n1 + n2;
}

int add_pointer(int* n1, int* n2) {
    *n1 = 3;
    *n2 = 4;
    return *n1 + *n2;
}

void add_pointer_with_no_result(int* n1, int* n2) {
    *n1 = 100;
    *n2 = 100;
}