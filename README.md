# The Power of JavaScript's ‘Map’ and ‘Set’ Objects

`Map` and `Set` objects can offer more efficient alternatives to plain objects or arrays in certain scenarios.

## Table of Contents
- [The Power of JavaScript's ‘Map’ and ‘Set’ Objects](#the-power-of-javascripts-map-and-set-objects)
  - [Table of Contents](#table-of-contents)
  - [Understanding Map](#understanding-map)
    - [Key Features of Map:](#key-features-of-map)
  - [Understanding Set](#understanding-set)
    - [Key Features of Set:](#key-features-of-set)
    - [When to Use Map and Set](#when-to-use-map-and-set)
      - [Use Map when:](#use-map-when)
      - [Use Set when:](#use-set-when)
    - [The Map and Set objects are powerful additions to JavaScript's standard library, offering more efficient and flexible ways to manage collections of data.](#the-map-and-set-objects-are-powerful-additions-to-javascripts-standard-library-offering-more-efficient-and-flexible-ways-to-manage-collections-of-data)
  - [**osot💤**](#osot)
    - [Connect with me:](#connect-with-me)


## Understanding Map

Imagine you’re working on an admin dashboard that tracks various actions performed by users, such as editing their profiles, uploading documents, or making transactions. The backend sends you an array of event logs for each user, with each log containing a timestamp and action details. Your task is to display only the most recent log for each action, as older logs are no longer relevant.

You can use Map to store the latest log for each action, with actionType as the key. As you iterate through the array of logs, you compare the timestamp of the current log with the one stored in the Map. If the current log is more recent, you update the Map with the new log.



```javascript
const eventLogs = [
    { userId: 'user123', 
      actionType: 'editProfile', 
      timestamp: '2024-08-10T10:15:30.000Z', 
      details: 'Changed avatar' 
      },
    { userId: 'user123', 
      actionType: 'uploadDocument', 
      timestamp: '2024-08-10T11:00:00.000Z', 
      details: 'Uploaded resume' 
      },
    { userId: 'user123', 
      actionType: 'editProfile', 
      timestamp: '2024-08-10T12:45:30.000Z', 
      details: 'Updated bio' 
      }, // Latest editProfile
    { userId: 'user123', 
      actionType: 'uploadDocument', 
      timestamp: '2024-08-10T12:00:00.000Z', 
      details: 'Uploaded cover letter' 
      }, // Latest uploadDocument
    { userId: 'user123', 
      actionType: 'makePurchase', 
      timestamp: '2024-08-10T13:30:45.000Z', 
      details: 'Purchased a subscription' 
      }
];

const latestEventLogMap = new Map();

eventLogs.forEach(log => {
    const currentLog = latestEventLogMap.get(log.actionType);

    if (!currentLog || new Date(log.timestamp) > new Date(currentLog.timestamp)) {
        latestEventLogMap.set(log.actionType, log);
    }
});

// Display the latest event logs
latestEventLogMap.forEach((log, actionType) => {
    console.log(`Action: ${actionType}, Timestamp: ${log.timestamp}, Details: ${log.details}`);
});
```

The Map object is a collection of key-value pairs where both keys and values can be of any data type. 

### Key Features of Map:
1. Key can be of any data type
2. No duplicate keys
3. The insertion order of keys is maintained.
4. Map has a size property that returns the number of entries.
5. Map is iterable, allowing easy looping through key-value pairs.
6. Map provides better performance for scenarios involving frequent additions and removals of key-value pairs.



```javascript
// Creating a new Map
const myMap = new Map();

// Setting key-value pairs
myMap.set('key', 'value');
myMap.set(123, 'another value');
myMap.set(true, 'boolean value');
myMap.set(true, 'another boolean value'); // The Map will only store one entry with the key true. The last entry,

// Retrieving values
console.log(myMap.get('key')); // Outputs: value
console.log(myMap.get(123)); // Outputs: another value
console.log(myMap.get(true)); // Outputs: boolean value

// Checking the size of the Map
console.log(myMap.size); // Outputs: 3

// Iterating over the Map
for (const [key, value] of myMap) {
    console.log(`${key}: ${value}`);
}
// Outputs:
// key: value
// 123: another value
// true: another boolean value
```


## Understanding Set

Imagine you’re working on an e-commerce analytics dashboard that tracks product purchases. The backend sends you a list of transactions, where each transaction includes a userId and the product purchased. Your goal is to create a list of unique users who have bought a specific product, even if some users have bought it more than once.

The Set object is ideal for this scenario because it automatically removes duplicates. By storing each userId in a Set, you can easily compile a list of unique users who have purchased the product.




```javascript
const transactions = [
    { userId: 'user1', productId: 'productA' },
    { userId: 'user2', productId: 'productA' },
    { userId: 'user1', productId: 'productA' }, // Duplicate purchase by user1
    { userId: 'user3', productId: 'productA' },
    { userId: 'user2', productId: 'productB' }, // Different product
    { userId: 'user4', productId: 'productA' }
];

// Store unique userIds in a Set
const uniquePurchasers = new Set();

transactions.forEach(transaction => {
    if (transaction.productId === 'productA') {
        uniquePurchasers.add(transaction.userId);
    }
});

// Display the list of unique users who bought productA
console.log([...uniquePurchasers]); // Outputs: ['user1', 'user2', 'user3', 'user4']
```

The Map object is a collection of key-value pairs where both keys and values can be of any data type. 

### Key Features of Set:
1. Each value in a Set must be unique.
2. Values can be of any data type.
3. Set has a size property that returns the number of values.
4. Set is iterable, allowing easy looping through its values.



```javascript
// Creating a new Set with initial values
const mySet = new Set([1, 2, 3, 3, 4]);

// Adding values
mySet.add(5);
mySet.add(2); // Duplicate value, will not be added

// Checking the size of the Set
console.log(mySet.size); // Outputs: 5

// Checking for the presence of a value
console.log(mySet.has(3)); // Outputs: true
console.log(mySet.has(10)); // Outputs: false

// Iterating over the Set
for (const value of mySet) {
    console.log(value);
}
// Outputs:
// 1
// 2
// 3
// 4
// 5

// Converting Set to an Array
const myArray = [...mySet];
console.log(myArray); // Outputs: [1, 2, 3, 4, 5]
```

### When to Use Map and Set
#### Use Map when:
- You need a collection of key-value pairs.
- You require keys that are not strings or symbols.
- You need to maintain the order of entries.
#### Use Set when:
- You need a collection of unique values.
- You want to efficiently check for the existence of a value.
- You need to eliminate duplicates from an array.



### The Map and Set objects are powerful additions to JavaScript's standard library, offering more efficient and flexible ways to manage collections of data.


---
> > ### ***"If you believe it, you can achieve it***
> > *The only limit to your success is your imagination"*



**osot💤**
---

### Connect with me:

<span style="font-size: 2rem; display: flex; gap: 2rem;">

  <a class="social-link" href="https://www.linkedin.com/in/ayo-osota/">
  <img src="./assets/linkedIn.svg" alt="linkedIn">
  </a>

  <a class="social-link" href="https://x.com/ayo_osota/">
  <img src="./assets/x.svg" alt="x">
  </a>

  <a class="social-link" href="mailto:osotaayomikun@gmail.com">
  <img src="./assets/mail.svg" alt="mail">
  </a>
</span>