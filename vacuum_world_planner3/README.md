Vacuum World Planner
=====
Problem Description
-----
Solve a robot planning problem through a simple grid environment where some
cells are contaminated with dirt. The robot can only move in the four cardinal
directions. All actions have the same cost and the objective is to achieve
a clean world with a plan of minimum cost.

Input Format
-----
4  
3  
\_\*\_\_  
\_\_#\*  
\_@\*#  

Number of columns and rows, given on separate lines. Then the world description.
The robot begins at @. A \_ is a blank cell. A \# is a blocked cell that the
robot can not enter. An \* is a dirty cell.
