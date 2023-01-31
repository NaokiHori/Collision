#include "common.h"
#include "boundary.h"
#include "internal.h"


void insert_bnode(bnode_t **bnode_root, bund_t *boundary){
  bnode_t *bnode_new = common_calloc(1, sizeof(bnode_t));
  bnode_new->boundary = boundary;
  bnode_new->next = *bnode_root;
  *bnode_root = bnode_new;
}

static void finalise_bnode(bnode_t *bnode){
  common_free(bnode->boundary);
  common_free(bnode);
}

void remove_all_bnodes(bnode_t *bnode_root){
  while(bnode_root){
    bnode_t *bnode_next = bnode_root->next;
    finalise_bnode(bnode_root);
    bnode_root = bnode_next;
  }
}

