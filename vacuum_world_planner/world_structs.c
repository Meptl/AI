#include <stdio.h>
#include <stdlib.h>

#include "world_structs.h"

int nodes_generated = 0;
int nodes_expanded = 0;

static char *get_stdin_line(void) {
    char *line = NULL;
    size_t size = 0;
    if (getline(&line, &size, stdin) == -1) {
        fprintf(stderr, "Early EOF");
        exit(-1);
    }
    return line;
}

world_t *init_world(int width, int height) {
    nodes_generated++;

    world_t *world = malloc(sizeof(*world));
    world->tiles = malloc(sizeof(tile *) * height);
    for (int i = 0; i < height; i++) {
        world->tiles[i] = malloc(sizeof(tile) * width);
    }

    world->width = width;
    world->height = height;
    world->total_dirt = 0;

    return world;
}

void free_world(world_t *world) {
    for (int i = 0; i < world->height; i++) {
        free(world->tiles[i]);
    } free(world->tiles);
    free(world);
}

// Read from stdin and fill world->tiles.
// world->tiles is referenced by
void populate_world(world_t *world) {
    int dirts = 0;
    for (int i = 0; i < world->height; i++) {
        char *line = get_stdin_line();
        for (int j = 0; j < world->width; j++) {
            char tile = line[j];
            switch(tile) {
                case '_':
                    world->tiles[i][j].state = CLEAN;
                    break;
                case '*':
                    world->tiles[i][j].state = DIRTY;
                    ++dirts;
                    break;
                case '@':
                    world->tiles[i][j].state = CLEAN;
                    world->vacuum_row = i;
                    world->vacuum_col = j;
                    break;
                case '#':
                    world->tiles[i][j].state = BLOCKED;
                    break;
                default:
                    fprintf(stderr, "Received unknown board tile.\n");
                    exit(-1);
                    break;
            }
        }
        free(line);
    }

    // Instantiate all costs to 1
    for (int i = 0; i < world->height; i++)
        for (int j = 0; j < world->width; j++)
            world->tiles[i][j].g = 1;

    world->total_dirt = dirts;
}

/* Returns bit vector of possible actions.
 * See enum action in vec.h
 */
int check_actions(world_t *world) {
    nodes_expanded++;

    int actions = 0;
    int row = world->vacuum_row;
    int col = world->vacuum_col;

    // Check the neighboring squares.
    // Assumes we begin in a valid state
    if ((row - 1) >= 0 && world->tiles[row - 1][col].state != BLOCKED) {
        nodes_generated++;
        actions |= NORTH;
    }
    if ((row + 1) < world->height && world->tiles[row + 1][col].state != BLOCKED) {
        nodes_generated++;
        actions |= SOUTH;
    }
    if ((col + 1) < world->width  && world->tiles[row][col + 1].state != BLOCKED) {
        nodes_generated++;
        actions |= EAST;
    }
    if ((col - 1) >= 0 && world->tiles[row][col - 1].state != BLOCKED) {
        nodes_generated++;
        actions |= WEST;
    }
    if (world->tiles[row][col].state == DIRTY) {
        nodes_generated++;
        actions |= VACUUM;
    }

    return actions;
}

/* Modifies the given world with the given action.
 */
world_t *world_do(world_t *world, action a) {
    switch(a) {
        case VACUUM:
            world->tiles[world->vacuum_row][world->vacuum_col].state = CLEAN;
            world->total_dirt--;
            break;
        case NORTH:
            world->vacuum_row = world->vacuum_row - 1;
            break;
        case SOUTH:
            world->vacuum_row = world->vacuum_row + 1;
            break;
        case EAST:
            world->vacuum_col = world->vacuum_col + 1;
            break;
        case WEST:
            world->vacuum_col = world->vacuum_col - 1;
            break;
        default:
            break;
    }
    return world;
}

void print_world(world_t *world) {
    for (int i = 0; i < world->height; i++) {
        for (int j = 0; j < world->width; j++) {
            if (i == world->vacuum_row && j == world->vacuum_col) {
                fprintf(stdout, "@");
                continue;
            }
            fprintf(stdout, "%01x", world->tiles[i][j].state);
        }
        fprintf(stdout, "\n");
    }
        fprintf(stdout, "\n");
}

/* Undoes the given action on the world.
 */
world_t *world_undo(world_t *world, action a) {
    switch(a) {
        case VACUUM:
            world->tiles[world->vacuum_row][world->vacuum_col].state = DIRTY;
            world->total_dirt++;
            break;
        case NORTH:
            world->vacuum_row = world->vacuum_row + 1;
            break;
        case SOUTH:
            world->vacuum_row = world->vacuum_row - 1;
            break;
        case EAST:
            world->vacuum_col = world->vacuum_col - 1;
            break;
        case WEST:
            world->vacuum_col = world->vacuum_col + 1;
            break;
        default:
            break;
    }
    return world;
}

world_t *world_copy(world_t *world) {
    world_t *cp = init_world(world->width, world->height);
    cp->total_dirt = world->total_dirt;
    cp->vacuum_row = world->vacuum_row;
    cp->vacuum_col = world->vacuum_col;

    for (int i = 0; i < world->height; i++)
        for (int j = 0; j < world->width; j++)
            cp->tiles[i][j] = world->tiles[i][j];

    return cp;
}
