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


Q:Anyway What is this?<br>
A: A rust based microservice that will take mouse events and give back a 
heatmap of usage.<br>


<b>Test command:</b><br>
curl -XPOST -H "Content-type: application/json" -d '{"x_vals": [0, 400, 700, 500],"y_vals": [0, 400, 700, 500], "x_res": 1920, "y_res": 1080}' '127.0.0.1:8000/heatmap/file/test'
<br>
<br>
The command above will give you a heatmap SVG image when working locally<br>
with a 1920x1080 resolusion.<br>
<br>
<b>What do I need to send?</b>

<ul>
    <li>"x_vals": [array elements here], "y_vals": [array elements here]</li>
    <li>"x_res": screenx here, "y_res": screeny here </li>
</ul>

<br>
That's pretty much it for using my microservice.













