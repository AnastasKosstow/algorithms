const int PRIMES_UP_TO = 100;

var primes = new List<int>();

for  (int number = 2; number <= PRIMES_UP_TO; number++)
{
    bool isPrime = true;
    for (int primeIndex = 0; primeIndex < Math.Sqrt(primes.Count); primeIndex++)
    {
        if (primes.Any() && number % primes[primeIndex] == 0)
        {
            isPrime = false;
            break;
        }
    }

    if (isPrime)
        primes.Add(number);
}
