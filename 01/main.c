#include <stdio.h>

int main() {
  FILE *fp = fopen("input", "r");
  int floor = 0;
  int c;

  while((c = getc(fp)) != EOF) {
    switch(c) {
    case '(': floor++; break;
    case ')': floor--; break;
    }
  }

  printf("Floor: %d\n", floor);
}
