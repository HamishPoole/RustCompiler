void main()
{
  int n;
  int i;
  boolean flag = true;
  putString("Enter value of N: ");
  (n=getInt());
  for ((i=2);((i<(n/2))&&flag);)
  {
    if ((((n/i)*i)==n))
      (flag=false);
    else
      (i=(i+1));
  }
  if (flag)
  {
    putInt(n);
    putStringLn(" is prime");
  }
  else
  {
    putInt(n);
    putString(" has ");
    putInt(i);
    putStringLn(" as a factor");
  }
}
