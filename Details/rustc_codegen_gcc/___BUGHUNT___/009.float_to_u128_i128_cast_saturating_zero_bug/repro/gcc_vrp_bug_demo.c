#include <stdbool.h>
#include <stdio.h>
#include <string.h>

unsigned int to_bits(float f){
	unsigned int ret;
	memcpy(&ret,&f, sizeof(unsigned int));
	return ret;
}
bool womky(void) {
  float val = 3.4028234663852885981170418348451692544e+38;
  float a = val * -(val * val);
  float b = val * a;
  float res = b * b;
  printf("%f %x\n",res, to_bits(res));
  return res > 0.0;
}
int main(void) { printf("%d\n", womky()); }
