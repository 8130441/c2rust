#include <stdio.h>
#include <stdint.h>

uint64_t ctr = 0;

uint64_t foo(void) {
    return 0x123457890ABCDEFULL;
}

uint64_t id(uint64_t x) {
    return x;
}

uint64_t deref(uint64_t *x) {
    return x ? *x : 0xDEADBEEF;
}

struct Foo {
    uint64_t n1, n2;
    struct Foo *p;
    uint64_t nn[2];
};

uint64_t my_Foo_hash(struct Foo x) {
    return (128 + x.n1) * 256 + (x.n2 + 128);
}

uint64_t double_hash(uint64_t a, uint64_t b) {
    return a * 65536 + b;
}

uint64_t fibo(uint64_t n[3], const uint64_t *p, const uint64_t *q, struct Foo foo) {
#if 0
    printf("fibo call %llu:%llu %p %p\n", ctr, n, p, q);
#endif
    ctr++;
    int nn = n[0];
    if (nn <= 1) {
        return 1;
    } else {
        struct Foo foo1 = { foo.n2, nn - 3, &foo1, { n[1], n[2] } };
        struct Foo foo2 = { nn - 3, nn - 4, &foo2, { n[1], n[2] } };
        uint64_t arr1[] = { foo.n1, foo1.n1, foo1.n2 };
        uint64_t arr2[] = { foo.n2, foo2.n1, foo2.n2 };
        return fibo(arr1, p, q, foo1) + fibo(arr2, q, p, foo2);
    }
}

int main() {
#if 0
    printf("Profiling test!!!\n");
    fibo(4);
#endif
    for (size_t i = 0; i < 5; i++) {
        struct Foo foo = { i - 1, i - 2, &foo, { i - 1, i - 2 } };
        uint64_t arr[] = { i, foo.n1, foo.n2 };
        fibo(arr, &i, NULL, foo);
#if 0
        printf("fibo(%zd)=%llu\n", i, fibo(i));
#endif
    }
    return 0;
}
