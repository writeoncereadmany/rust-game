<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.11.0" name="Walls" tilewidth="12" tileheight="12" tilecount="20" columns="4">
 <image source="../graphics/tilesheet.png" width="48" height="60"/>
 <tile id="0" type="Wall"/>
 <tile id="1" type="Wall"/>
 <tile id="2" type="Wall"/>
 <tile id="3" type="Wall"/>
 <tile id="4" type="Wall"/>
 <tile id="5" type="Wall"/>
 <tile id="6" type="Wall"/>
 <tile id="7" type="Wall"/>
 <tile id="8" type="Wall"/>
 <tile id="9" type="Wall"/>
 <tile id="10" type="Wall"/>
 <tile id="11" type="Wall"/>
 <tile id="12" type="Wall"/>
 <tile id="13" type="Wall"/>
 <tile id="14" type="Wall"/>
 <tile id="15" type="Wall"/>
 <tile id="16" type="Ledge"/>
 <tile id="17" type="Ledge"/>
 <tile id="18" type="Ledge"/>
 <tile id="19" type="Ledge"/>
 <wangsets>
  <wangset name="Walls" type="edge" tile="-1">
   <wangcolor name="Wall" color="#ff0000" tile="5" probability="1"/>
   <wangcolor name="Ledge" color="#00ff00" tile="17" probability="1"/>
   <wangtile tileid="0" wangid="0,0,1,0,1,0,0,0"/>
   <wangtile tileid="1" wangid="0,0,1,0,1,0,1,0"/>
   <wangtile tileid="2" wangid="0,0,0,0,1,0,1,0"/>
   <wangtile tileid="3" wangid="0,0,0,0,1,0,0,0"/>
   <wangtile tileid="4" wangid="1,0,1,0,1,0,0,0"/>
   <wangtile tileid="5" wangid="1,0,1,0,1,0,1,0"/>
   <wangtile tileid="6" wangid="1,0,0,0,1,0,1,0"/>
   <wangtile tileid="7" wangid="1,0,0,0,1,0,0,0"/>
   <wangtile tileid="8" wangid="1,0,1,0,0,0,0,0"/>
   <wangtile tileid="9" wangid="1,0,1,0,0,0,1,0"/>
   <wangtile tileid="10" wangid="1,0,0,0,0,0,1,0"/>
   <wangtile tileid="11" wangid="1,0,0,0,0,0,0,0"/>
   <wangtile tileid="12" wangid="0,0,1,0,0,0,0,0"/>
   <wangtile tileid="13" wangid="0,0,1,0,0,0,1,0"/>
   <wangtile tileid="14" wangid="0,0,0,0,0,0,1,0"/>
   <wangtile tileid="16" wangid="0,0,2,0,0,0,0,0"/>
   <wangtile tileid="17" wangid="0,0,2,0,0,0,2,0"/>
   <wangtile tileid="18" wangid="0,0,0,0,0,0,2,0"/>
  </wangset>
 </wangsets>
</tileset>
