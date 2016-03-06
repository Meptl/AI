#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "vec.h"
#include "world_structs.h"
#include "world_list.h"

// Ensure A is true, otherwise print M(formatted) and goto error label
#define check(A, M, args...)            \
    if (!(A)) {                         \
        fprintf(stderr, M"\n", ##args); \
        goto error;                     \
    }
#define DEBUG 1
#if !DEBUG
#define log(M, ...)
#else
#define log(M, ...) \
    fprintf(stderr, "DEBUG %s:%d:%s: " M "\n", \
            __FILE__, __LINE__, __func__, ##__VA_ARGS__)
#endif

extern int nodes_generated;
extern int nodes_expanded;

char *get_stdin_line(void) {
    char *line = NULL;
    size_t size = 0;
    if (getline(&line, &size, stdin) == -1) {
        fprintf(stderr, "Early EOF");
        exit(-1);
    }
    return line;
}

void print_usage_and_exit(void) {
    fprintf(stderr, "Usage: ./a1 [SEARCH]\n"
                  "\tavailable searches are uniform-cost and depth-first\n");
    exit(-1);
}

int world_dup_check(world_t *world, world_list *closed) {
    // Save position and world dirts
    int curr_row = world->vacuum_row;
    int curr_col = world->vacuum_col;
    int num_dirts = world->total_dirt;

    node *view = closed->head;
    while (view) {
        if (view->tar->vacuum_row == curr_row &&
            view->tar->vacuum_col == curr_col &&
            view->tar->total_dirt == num_dirts)
            return 1;
    }

    return 0;
}

// Returns 1 on duplicate and 0 on not duplicate
int world_cycle_check(world_t *world, vector *solution) {
    // Save position and world dirts
    int curr_row = world->vacuum_row;
    int curr_col = world->vacuum_col;
    int num_dirts = world->total_dirt;

    for (int i = solution->length - 1; i >= 0; i--) {
        action a = solution->values[i];
        world_undo(world, a);
        if (world->vacuum_row == curr_row &&
            world->vacuum_col == curr_col &&
            world->total_dirt == num_dirts)
            return 1;
    }

    // Revert to current state
    for (int i = 0; i < solution->length; i++) {
        action action = solution->values[i];
        world_do(world, action);
    }
    return 0;
}


// Returns 1 for done, 0 for reached limit/duplicate state.
int uniform_cost(world_list *open, world_list *closed) {
    log("test");
    world_t *world = open->head->tar;
    if (world->total_dirt == 0)
        return 1;

    if (world_dup_check(world, closed)) {
        return 0;
    }

    world_list_pop_front(open);
    world_list_push_front(closed, world);
    int actions = check_actions(world);
    // The sorrows of C enums below.
    if (actions & VACUUM) {
        world_t *new_state = world_copy(world);
        world_do(new_state, VACUUM);
        world_list_push_back(open, new_state);
    }
    if (actions & WEST) {
        world_t *new_state = world_copy(world);
        world_do(new_state, WEST);
        world_list_push_back(open, new_state);
    }
    if (actions & SOUTH) {
        world_t *new_state = world_copy(world);
        world_do(new_state, SOUTH);
        world_list_push_back(open, new_state);
    }
    if (actions & EAST) {
        world_t *new_state = world_copy(world);
        world_do(new_state, EAST);
        world_list_push_back(open, new_state);
    }
    if (actions & NORTH) {
        world_t *new_state = world_copy(world);
        world_do(new_state, NORTH);
        world_list_push_back(open, new_state);
    }

    if (uniform_cost(open, closed) == 1)
        return 1;
    return 0;
}

// Returns 1 for done, 0 for reached limit/duplicate state.
int depth_first(world_t *world, vector *solution) {
    //print_world(world);

    // Check if world is done.
    if (world->total_dirt == 0)
        return 1;

    if (world_cycle_check(world, solution) == 1) {
        return 0;
    }


    int actions = check_actions(world);
    // The sorrows of C enums below.
    if (actions & VACUUM) {
        vec_push(solution, VACUUM);
        if (depth_first(world_do(world, VACUUM), solution) == 1)
            return 1;

        world_undo(world, VACUUM);
        vec_pop(solution);
    }
    if (actions & WEST) {
        vec_push(solution, WEST);
        if (depth_first(world_do(world, WEST), solution) == 1)
            return 1;

        world_undo(world, WEST);
        vec_pop(solution);
    }
    if (actions & SOUTH) {
        vec_push(solution, SOUTH);
        if (depth_first(world_do(world, SOUTH), solution) == 1)
            return 1;

        world_undo(world, SOUTH);
        vec_pop(solution);
    }
    if (actions & EAST) {
        vec_push(solution, EAST);
        if (depth_first(world_do(world, EAST), solution) == 1)
            return 1;

        world_undo(world, EAST);
        vec_pop(solution);
    }
    if (actions & NORTH) {
        vec_push(solution, NORTH);
        if (depth_first(world_do(world, NORTH), solution) == 1)
            return 1;

        world_undo(world, NORTH);
        vec_pop(solution);
    }

    return 0;
}

vector *search_uniform_cost(world_t *world) {
    vector *solution = vec_init(world->width * world->height);
    world_list *open = world_list_init();
    world_list *closed = world_list_init();

    world_list_push_front(open, world);

    //int i = uniform_cost(open, closed);
    int i = 0;
    if (i == 0)
        fprintf(stdout, "No solution\n");

    world_list_free(open);
    world_list_free(closed);
    return solution;
}

vector *search_depth_first(world_t *world) {
    vector *solution = vec_init(world->width * world->height);
    int i = depth_first(world, solution);
    if (i == 0)
        fprintf(stdout, "No solution\n");
    return solution;
}

int main(int argc, char *argv[])
{
    if (argc != 2)
        print_usage_and_exit();

    int uniform_cost;
    if (strcmp(argv[1], "uniform-cost") == 0)
        uniform_cost = 1;
    else if (strcmp(argv[1], "depth-first") == 0)
        uniform_cost = 0;
    else
        print_usage_and_exit();


    // get_stdin_line is declared in world_structs.c
    char *line = get_stdin_line();
    int width = atoi(line);
    free(line);

    line = get_stdin_line();
    int height = atoi(line);
    free(line);

    world_t *world = init_world(width, height);
    populate_world(world);

    vector *solution;
    if (uniform_cost)
        solution = search_uniform_cost(world);
    else
        solution = search_depth_first(world);

    for (int i = 0; i < solution->length; i++) {
        switch(solution->values[i]) {
            case VACUUM:
                fprintf(stdout, "V\n");
                break;
            case NORTH:
                fprintf(stdout, "N\n");
                break;
            case SOUTH:
                fprintf(stdout, "S\n");
                break;
            case EAST:
                fprintf(stdout, "E\n");
                break;
            case WEST:
                fprintf(stdout, "W\n");
                break;
            default:
                break;
        }
    }
    fprintf(stdout, "%d nodes generated\n", nodes_generated);
    fprintf(stdout, "%d nodes expanded\n", nodes_expanded);

    vec_free(solution);
    free_world(world);
    return 0;
}

