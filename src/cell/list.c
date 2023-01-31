#include <stdbool.h>
#include "common.h"
#include "cell.h"


static bool find_node(cnode_t **cnode_root, const cell_t *cell){
  // return true  when the cell is found in the list
  // return false otherwise
  for(const cnode_t *cnode = *cnode_root; cnode; cnode = cnode->next){
    if(cnode->cell == cell){
      return true;
    }
  }
  return false;
}

int insert_cnode(cnode_t **cnode_root, cell_t *cell){
  // do nothing and return if the cell already exists
  if(find_node(cnode_root, cell)){
    return 1;
  }
  // create new node storing cell as info
  cnode_t *cnode_new = common_calloc(1, sizeof(cnode_t));
  cnode_new->cell = cell;
  // insert the new node to the head of linked list
  cnode_new->next = *cnode_root;
  *cnode_root = cnode_new;
  return 0;
}

static void finalise_cnode(cnode_t *cnode){
  common_free(cnode);
}

int remove_cnode(cnode_t **cnode_root, const cell_t *cell){
  while(*cnode_root){
    if(cell == (*cnode_root)->cell){
      // keep next node
      cnode_t *cnode_next = (*cnode_root)->next;
      // clean-up pointer of current node
      finalise_cnode(*cnode_root);
      // update connection
      *cnode_root = cnode_next;
      return 0;
    }
    cnode_root = &((*cnode_root)->next);
  }
  // not found
  return 1;
}

void remove_all_cnodes(cnode_t *cnode_root){
  while(cnode_root){
    cnode_t *cnode_next = cnode_root->next;
    finalise_cnode(cnode_root);
    cnode_root = cnode_next;
  }
}

