<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DZ</name>
   <tag></tag>
   <elementGuidId>3021aedf-8598-4d80-ad72-a29fbeaf551b</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=BARRIO%2025%20DE%20MAYO&amp;level4=POSADAS&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;postalCode=3300</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.AddressJson</defaultValue>
      <description></description>
      <id>f056fb45-d09d-487e-a6c8-2f879e4e4c6b</id>
      <masked>false</masked>
      <name>AddressJson</name>
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



WS.verifyElementPropertyValue(response, 'status', &quot;DU&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;DZ&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.geoType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level2', &quot;MISIONES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level3', &quot;CAPITAL&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level4', &quot;POSADAS&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;BARRIO 25 DE MAYO&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-27.368742&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-55.894215&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;3300&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;2&quot;)

assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;MISIONES&quot;,&quot;level3&quot;:&quot;CAPITAL&quot;,&quot;level4&quot;:&quot;POSADAS&quot;,&quot;level5&quot;:&quot;25 DE MAYO&quot;,&quot;street&quot;:&quot;Barrio 25 DE MAYO&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;latitude&quot;:&quot;-27.392435&quot;,&quot;longitude&quot;:&quot;-55.902976&quot;,&quot;postalCode&quot;:&quot;3300&quot;,&quot;fromStreetNumber&quot;:&quot;0&quot;,&quot;toStreetNumber&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;MISIONES&quot;,&quot;level3&quot;:&quot;CAPITAL&quot;,&quot;level4&quot;:&quot;POSADAS&quot;,&quot;level5&quot;:&quot;1 DE MAYO&quot;,&quot;street&quot;:&quot;Barrio 1 DE MAYO&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;latitude&quot;:&quot;-27.362872&quot;,&quot;longitude&quot;:&quot;-55.925473&quot;,&quot;postalCode&quot;:&quot;3300&quot;,&quot;fromStreetNumber&quot;:&quot;0&quot;,&quot;toStreetNumber&quot;:&quot;0&quot;}')

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-55.894215&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-27.368742&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-55.894215&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;6&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;6&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-27.368742&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
