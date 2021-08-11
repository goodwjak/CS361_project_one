# CS361_project_one


<b>USAGE:</b>

To use the Microservice for heatmaps:<br>
1.) start the micro service in the dir with "cargo run"<br>
2.) use JS to record mouse x,y data on clicks into a array.<br>
3.) POST the data to the microservice using the format below:<br>

4.)Get a heatmap back for your use.<br>

The needed JS is in the script file, Once you add this to the head of your
html document you can use it to post the mouse event data to the heatmap
microservice.<br>


Going to add the code I made in my gitlab repo to this one.
I kinda wish I would have remembered I had this one a gitlab I was supposed to
use.

Q:Anyway What is this?<br>
A: A rust based microservice that will take mouse events and give back a 
heatmap of usage.<br>


<b>Test command:</b><br>
curl -XPOST -H "Content-type: application/json" -d '{"x_vals": [0, 1, 2, 3],"y_vals": [2, 4, 9, 1]}' '127.0.0.1:8000/heatmap/test'












