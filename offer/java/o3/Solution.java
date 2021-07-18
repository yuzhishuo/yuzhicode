package 剑指offer.o3;

import java.util.ArrayList;
import java.util.HashMap;

public class Solution {

  /**
   * 返回数组中重复出现的数字
   * @param nums
   * @return
   */
  public int findRepeatNumber(int[] nums) {
    HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
    ArrayList<Integer> repeatNum = new ArrayList<Integer>();
    for (int num:nums){
      int count = map.getOrDefault(num, 0);
      map.put(num, count+1);
      if (map.get(num)>1){
        repeatNum.add(num);
      }
    }
    if (repeatNum.size()==0){
      return -1;
    }else{
      return repeatNum.get(0);
    }
  }
}
