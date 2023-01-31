#include <stdbool.h>
#include "common.h"
#include "particle.h"


static bool find_node(const pnode_t *pnode_root, const part_t *particle){
  // return true  when the particle is found in the list
  // return false otherwise
  for(const pnode_t *pnode = pnode_root; pnode; pnode = pnode->next){
    if(pnode->particle == particle){
      return true;
    }
  }
  return false;
}

int insert_pnode(pnode_t **pnode_root, part_t *particle){
  // do nothing and return if the particle already exists
  if(find_node(*pnode_root, particle)){
    return 1;
  }
  // create new node storing particle
  pnode_t *pnode_new = common_calloc(1, sizeof(pnode_t));
  pnode_new->particle = particle;
  // order does not matter for now
  // insert the new node to the head of linked list
  pnode_new->next = *pnode_root;
  *pnode_root = pnode_new;
  return 0;
}

static void finalise_pnode(pnode_t *pnode){
  common_free(pnode);
}

int remove_pnode(pnode_t **pnode_root, const part_t *particle){
  while(*pnode_root){
    if(particle == (*pnode_root)->particle){
      // keep next node
      pnode_t *pnode_next = (*pnode_root)->next;
      // clean-up pointer of current node
      finalise_pnode(*pnode_root);
      // update connection
      *pnode_root = pnode_next;
      return 0;
    }
    pnode_root = &((*pnode_root)->next);
  }
  // not found
  return 1;
}

void remove_all_pnodes(pnode_t *pnode_root){
  while(pnode_root){
    pnode_t *pnode_next = pnode_root->next;
    finalise_pnode(pnode_root);
    pnode_root = pnode_next;
  }
}

