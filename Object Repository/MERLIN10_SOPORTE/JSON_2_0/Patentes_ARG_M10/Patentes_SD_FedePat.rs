<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Patentes_SD_FedePat</name>
   <tag></tag>
   <elementGuidId>5e686484-fd15-4a39-bda5-6ac9d1366204</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded; charset=utf-8</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${PatentesJson}?clientAccessCode=ea86ad3f0ed110b6961d1b883688534c&amp;domain=AA004FG</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PatentesJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>PatentesJson</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'status', &quot;SD&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.domain', &quot;AA004FG&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.vehicleBrand', &quot;TOYOTA&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.model', &quot;ETIOS X 1.5 M/T&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.motor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.chassis', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.fabricationPlace', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.year', &quot;2016&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
