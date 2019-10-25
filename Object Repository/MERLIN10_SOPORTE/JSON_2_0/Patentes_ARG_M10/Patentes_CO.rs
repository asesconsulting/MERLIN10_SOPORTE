<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Patentes_CO</name>
   <tag></tag>
   <elementGuidId>af82fdcf-3f81-4c1b-a90e-e54e05a63c9f</elementGuidId>
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
   <restUrl>${PatentesJson}?clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;domain=EYD785</restUrl>
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



WS.verifyElementPropertyValue(response, 'status', &quot;CO&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.domain', &quot;EYD785&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.vehicleBrand', &quot;VOLKSWAGEN&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.model', &quot;POLO CLASSIC 1.9L SD 31A&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.motor', &quot;1Y 961787&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.chassis', &quot;8AWZZZ9EZ5A699109&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.fabricationPlace', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nVehicle.year', &quot;2005&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
