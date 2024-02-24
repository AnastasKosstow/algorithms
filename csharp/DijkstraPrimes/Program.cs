const int PRIMES_UP_TO = 30;


var primes = new List<int>();
var pool = new Pool();

primes.Add(2);
pool.Insert(2);

for (int number = 3; number < PRIMES_UP_TO; number++)
{
    var smallestMultiplier = pool.GetSmallestMultiplier();
    if (smallestMultiplier > 0)
    {
        if (number < smallestMultiplier)
        {
            primes.Add(number);
            pool.Insert(number);
        }
        else
        {
            pool.Update(number);
        }
    }
}

readonly struct Pool
{
    private readonly List<(int Prime, int Multiple)> pairs;

    public Pool()
    {
        pairs = new();
    }

    public readonly void Insert(int value)
        =>
        this.pairs.Add((value, value * value));

    public readonly int GetSmallestMultiplier()
        =>
        this.pairs.Count > 0
            ? this.pairs[0].Multiple
            : 0;

    public void Update(int value)
    {
        int index = 0;
        for (; index < pairs.Count; index++)
        {
            if (pairs[index].Multiple == value)
            {
                pairs[index] = (pairs[index].Prime, pairs[index].Multiple + pairs[index].Prime);
            }
            else
            {
                break;
            }
        }

        if (this.pairs.Count > 1)
        {
            while (index >= 0)
            {
                index -= 1;
                int i = index;
                while (this.pairs[i].Multiple > this.pairs[i + 1].Multiple)
                {
                    (this.pairs[i + 1], this.pairs[i]) = (this.pairs[i], this.pairs[i + 1]);
                    i += 1;
                }
                if (index == 0)
                    break;
            }
        }
    }
}
