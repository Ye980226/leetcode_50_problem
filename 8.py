class Solution:
    def myAtoi(self, s: str) -> int:
        s=s.strip()
        result=""
        count=0
        flag=False
        sign=""
        for i in s:
            if i.isdigit():
                result+=i
                flag=True
            elif (i=="+" or i=="-") and count==0 and not flag:
                sign=i
                count+=1
            else:
                break
            if (flag or count>1) and not i.isdigit():
                # print("break")
                break
        if result=="":
            return 0
        else:
            if sign =="-":
                result="-"+result
            result=int(result)
            if result>2**31-1:
                return 2**31-1
            elif result<-2**31:
                return -2**31
            else :
                return result
s=Solution()
print(s.myAtoi("+-12"))
print(s.myAtoi(".1"))
print(s.myAtoi(" 1"))
print(s.myAtoi(" .1"))
print(s.myAtoi("3.1415926"))
print(s.myAtoi(" -42"))
print(s.myAtoi("4193 with words"))
print(s.myAtoi("words and 987"))
print(s.myAtoi("-91283472332"))
print(s.myAtoi("00000-42a1234"))
print(s.myAtoi("-13+8"))
print(s.myAtoi("123-"))
