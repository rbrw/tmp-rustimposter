
#include <stdio.h>

#include "max.h"

int
main(int argc, char **argv)
{
  const int n = 4;
  double doubles[4] = {3, 1, 11, 7};
  const double max = doubles_max(doubles, n);
  printf("max: %f\n", max);
  return 0;
}
