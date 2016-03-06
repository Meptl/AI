#ifndef WORLD_STRUCTS_H
#define WORLD_STRUCTS_H

typedef enum action {
    NORTH  = 0x01,
    SOUTH  = 0x02,
    EAST   = 0x04,
    WEST   = 0x08,
    VACUUM = 0x10,
} action;
typedef enum {CLEAN, DIRTY, BLOCKED} tile_state;


typedef struct tile {
    tile_state state;
    int g;
} tile;

typedef struct world_t {
    tile **tiles;
    int height;
    int width;
    int vacuum_row;
    int vacuum_col;
    int total_dirt;
} world_t;

world_t *init_world(int, int);
void free_world(world_t *);
void populate_world(world_t *);
int check_actions(world_t *);
world_t *world_do(world_t *, action);
world_t *world_undo(world_t *, action);
void print_world(world_t *);
world_t *world_copy(world_t *);

#endif
