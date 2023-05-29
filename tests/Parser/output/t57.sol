void main()
{
  int n;
  int i;
  int current;
  int next;
  int twoaway;
  putString("How many Fibonacci numbers do you want to compute? ");
  getInt(n);
  if ((n<=0))
    putString("The number should be positive.\n");
  else
  {
    putString("\n\n\tI \t Fibonacci(I) \n\t=====================\n");
    (next=(current=1));
    for ((i=1);(i<=n);(i=(i+1)))
    {
      putString("\t");
      putInt(i);
      putString("\t");
      putIntLn(current);
      (twoaway=(current+next));
      (current=next);
      (next=twoaway);
    }
  }
}
