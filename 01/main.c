#include <stdio.h>

int main() {
  FILE *fp = fopen("input", "r");
  int pos = 0;
  int floor = 0;
  int c;

  while((c = getc(fp)) != EOF) {
    switch(c) {
    case '(': floor++; break;
    case ')': floor--; break;
    }

    pos++;

    if(floor == -1) {
      printf("Char at possition: %d\n", pos); break;
    }
  }
}
