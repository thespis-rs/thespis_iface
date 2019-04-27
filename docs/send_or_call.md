# To Send or To Call?

Advantages of send:
- original recipient needs not be the one to respond, can forward message to another handler and this is transparent
- lower overhead when requester is not the one needing an answer
- call is 3x as slow as send, even though it's just 2 ways

Advantages of Call:
- automatic acknowledgement of the message having been handled
- no need to do connection tracking manually
- disadvantage: how to implement await! with timeout?
