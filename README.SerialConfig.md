### KVM Virtual Machine Serial Port Configuration
Add Serial 
```xml
<serial type="unix">
  <source mode="bind" path="/tmp/gt_com"/>
  <target type="isa-serial" port="1">
    <model name="isa-serial"/>
  </target>
  <alias name="serial1"/>
</serial>     
```

```bash
sudo virsh dumpxml RX-GranuTools | grep -i serial    
```
