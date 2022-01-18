#include <stdio.h>
#include <stdlib.h>

int int_comp(const void* a, const void* b) {
  int arg1 = *(const int*) a;
  int arg2 = *(const int*) b;
  
  return (arg1 > arg2) - (arg1 < arg2);
}

int main() {
  FILE *fp = fopen("input", "r");
  int total_paper_feet = 0;
  int total_ribbon_feet = 0;
  int s[3];   // surface
  int ss[3];  // [0] = l * w

  while(fscanf(fp, "%dx%dx%d", &s[0], &s[1], &s[2]) != EOF) {
    ss[0] = s[0] * s[1];
    ss[1] = s[1] * s[2];
    ss[2] = s[2] * s[0];

    qsort(s, 3, sizeof(int), int_comp);
    qsort(ss, 3, sizeof(int), int_comp);

    total_paper_feet += 2*ss[0] + 2*ss[1] + 2*ss[2] + ss[0];
    total_ribbon_feet += 2*s[0] + 2*s[1] + s[0]*s[1]*s[2];
  }

  printf("Total square feet of wrapping paper: %d\n", total_paper_feet);
  printf("Toal of ribbon: %d\n", total_ribbon_feet);
}
