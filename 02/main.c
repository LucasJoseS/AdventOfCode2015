#include <stdio.h>

int main() {
  FILE *fp = fopen("input", "r");
  int l, w, h, lw, wh, hl, square, small, total = 0;

  while(fscanf(fp, "%dx%dx%d", &l, &w, &h) != EOF) {
    lw = l*w;
    wh = w*h;
    hl = h*l;
    square = 2*lw + 2*wh + 2*hl;

    small = lw;
    if(small > wh) small = wh;
    if(small > hl) small = hl;

    total += small + square;
  }

  printf("Total square feet of wrapping paper: %d\n", total);
}
