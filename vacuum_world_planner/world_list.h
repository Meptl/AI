#ifndef WORLD_LIST_H
#define WORLD_LIST_H

struct elem {
    world_t *tar;
    struct elem *next;
};

typedef struct elem node;

typedef struct world_list {
    node *head;
    node *tail;
} world_list;

world_list *world_list_init(void);
void world_list_free(world_list *wl);
world_list *world_list_push_front(world_list *wl, world_t *world);
world_list *world_list_push_back(world_list *wl, world_t *world);
world_t *world_list_pop_front(world_list *wl);

#endif
