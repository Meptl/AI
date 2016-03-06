#include <stdlib.h>
#include "world_structs.h"
#include "world_list.h"

world_list *world_list_init(void) {
    world_list *wl = malloc(sizeof(*wl));
    wl->head = NULL;
    wl->tail = NULL;

    return wl;
}

void world_list_free(world_list *wl) {
    node *view = wl->head;

    while (view != NULL) {
        node *prev = view;
        view = view->next;
        free(prev);
    }
    free(wl);
}

world_list *world_list_push_front(world_list *wl, world_t *world) {
    node *new_node = malloc(sizeof(*new_node));
    new_node->tar = world;

    if (wl->head == NULL) {//Uninitialized list.
        wl->head = new_node;
        wl->tail = new_node;
        new_node->next = NULL;
        return wl;
    }

    new_node->next = wl->head;
    wl->head = new_node;
    return wl;
}

world_list *world_list_push_back(world_list *wl, world_t *world) {
    node *new_node = malloc(sizeof(*new_node));
    new_node->tar = world;
    new_node->next = NULL;

    if (wl->head == NULL) {//Uninitialized list.
        wl->head = new_node;
        wl->tail = new_node;
        return wl;
    }

    wl->tail->next = new_node;
    wl->tail = new_node;
    return wl;
}

world_t *world_list_pop_front(world_list *wl) {
    node *view = wl->head;
    wl->head = view->next;

    world_t *view_world = view->tar;
    free(view);
    return view_world;
}
