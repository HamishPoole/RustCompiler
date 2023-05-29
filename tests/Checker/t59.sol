void printIntArray(int a[], int n)
{
  int i;
  for ((i=0);(i<n);(i=(i+1)))
  {
    putInt(a[i]);
    putString(" ");
  }
  putLn();
}
void bubbleSort(int a[], int n)
{
  int lcv;
  int limit = (n-1);
  int temp;
  int lastChange;
  while (limit)
  {
    (lastChange=0);
    for ((lcv=0);(lcv<limit);(lcv=(lcv+1)))
      if ((a[lcv]>a[(lcv+1)]))
      {
        (temp=a[lcv]);
        (a[lcv]=a[(lcv+1)]);
        (a[(lcv+1)]=temp);
        (lastChange=lcv);
      }
    (limit=lastChange);
  }
}
void main()
{
  int x[10] = {3,10,1,5,8,0,20,1,4,100};
  int hmny = 10;
  int who;
  int where;
  putStringLn("The array was:");
  printIntArray(x, hmny);
  bubbleSort(x, hmny);
  putStringLn("The sorted array is:");
  printIntArray(x, hmny);
}
