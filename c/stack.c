#include <stdio.h>
#include <stdlib.h>

typedef struct Node {
    void *value;
    struct Node *next;
} T_Node;

typedef struct Stack {
    T_Node *top;
    unsigned int size;    
} T_Stack;

T_Stack* stack_new() {
    T_Stack *new_stack = (T_Stack*)malloc(sizeof(T_Stack));
    if (new_stack == NULL) {
        fprintf(stderr, "Failed to allocate memory for stack!");
        return NULL;
    }

    new_stack->top = NULL;
    new_stack->size = 0;

    return new_stack;
}

T_Node *node_new(void *value) {
    T_Node *new_node = (T_Node*)malloc(sizeof(T_Node));
    if (new_node == NULL) {
        fprintf(stderr, "Failed to allocate memory for node!");
        return NULL;
    }

    new_node->value = value;
    new_node->next = NULL;

    return new_node;
}

int stack_push(T_Stack *stack, void *value) {
    T_Node *node = node_new(value);
    if (node == NULL) {
        return 1;
    }

    if (stack->size > 0) {
        node->next = stack->top;
    }

    stack->top = node;
    stack->size += 1;

    return 0;
}

int stack_pop(T_Stack *stack) {
    if (stack->size == 0) {
        fprintf(stderr, "Failed to pop stack because stack is empty!");
        return 1;
    }

    T_Node *top = stack->top;
    stack->top = stack->top->next;
    stack->size -= 1;

    free(top->value);
    free(top);

    return 0;
}

void *stack_top(T_Stack *stack) {
    if (stack->size == 0) {
        fprintf(stderr, "Failed to get value of top of stack because stack is empty!");
        return NULL;
    }

    return stack->top->value;    
}

int stack_empty(T_Stack *stack) {
    return stack->size == 0;
}

void stack_clear(T_Stack *stack) {
    while (!stack_empty(stack)) {
        stack_pop(stack);
    }
}

void stack_drop(T_Stack *stack) {
    free(stack);
}

int main() {
    T_Stack *stack = stack_new();
    if (stack == NULL) {
        return 1;
    }

    for (int i = 0; i < 10; ++i) {
        int *num = (int*)malloc(sizeof(int));
        *num = i;
        stack_push(stack, (void*)num);
    }

    while (!stack_empty(stack)) {
        void *top = stack_top(stack);
        stack_pop(stack);

        fprintf(stdout, "%d ", *(int*)top);
    }

    stack_drop(stack);

    return 0;    
}