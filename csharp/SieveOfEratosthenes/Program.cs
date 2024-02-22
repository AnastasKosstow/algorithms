const int PRIMES_UP_TO = 100;

var sieve = Enumerable.Repeat(true, PRIMES_UP_TO).ToArray();
sieve[0] = false;
sieve[1] = false;

for (int number = 0; number < Math.Sqrt(PRIMES_UP_TO); number++)
{
    if (sieve[number] == true) 
    {
        var multiple = number * number;
        while (multiple < PRIMES_UP_TO) 
        {
            sieve[multiple] = false;
            multiple += number;
        }
    }
}

var primes = new List<int>();
for (int index = 0; index < sieve.Length; index++)
{
    if (sieve[index])
        primes.Add(index);

}
