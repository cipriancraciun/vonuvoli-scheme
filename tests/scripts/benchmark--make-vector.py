
import time

timestamp_start = time.clock ()

loop = 100 * 1000
while loop > 0 :
	loop = loop - 1
	
	[0] * 100

print int ((time.clock () - timestamp_start) * 1000)

