<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DC</name>
   <tag></tag>
   <elementGuidId>217ce505-fa77-4748-997f-141235638477</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=barrio%20MASCHWITZ%201387&amp;level4=INGENIERO%20MASCHWITZ&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf&amp;postalCode=1623</restUrl>
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
WS.verifyElementPropertyValue(response, 'statusReason', &quot;DC&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.geoType', &quot;6&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level2', &quot;BUENOS AIRES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level3', &quot;ESCOBAR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level4', &quot;INGENIERO MASCHWITZ&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;BARRIO MASCHWITZ 1387&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-34.383353&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-58.750027&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;1623&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;0&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;1387&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;3&quot;)

assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;ESCOBAR&quot;,&quot;level4&quot;:&quot;MAQUINISTA F SAVIO&quot;,&quot;level5&quot;:&quot;MASCHWITZ PRIVADO&quot;,&quot;street&quot;:&quot;BO MASCHWITZ PRIVADO&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;latitude&quot;:&quot;-34.392926&quot;,&quot;longitude&quot;:&quot;-58.766695&quot;,&quot;postalCode&quot;:&quot;1619&quot;,&quot;fromStreetNumber&quot;:&quot;0&quot;,&quot;toStreetNumber&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;ESCOBAR&quot;,&quot;level4&quot;:&quot;INGENIERO MASCHWITZ&quot;,&quot;level5&quot;:&quot;MASCHWITZ VILLAGE&quot;,&quot;street&quot;:&quot;BO MASCHWITZ VILLAGE&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;latitude&quot;:&quot;-34.373931&quot;,&quot;longitude&quot;:&quot;-58.745573&quot;,&quot;postalCode&quot;:&quot;1619&quot;,&quot;fromStreetNumber&quot;:&quot;0&quot;,&quot;toStreetNumber&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;geoType&quot;:&quot;8&quot;,&quot;level1&quot;:&quot;&quot;,&quot;level2&quot;:&quot;BUENOS AIRES&quot;,&quot;level3&quot;:&quot;ESCOBAR&quot;,&quot;level4&quot;:&quot;INGENIERO MASCHWITZ&quot;,&quot;level5&quot;:&quot;MASCHWITZ CLUB&quot;,&quot;street&quot;:&quot;BO MASCHWITZ CLUB&quot;,&quot;houseNumber&quot;:&quot;0&quot;,&quot;latitude&quot;:&quot;-34.377518&quot;,&quot;longitude&quot;:&quot;-58.754644&quot;,&quot;postalCode&quot;:&quot;1619&quot;,&quot;fromStreetNumber&quot;:&quot;0&quot;,&quot;toStreetNumber&quot;:&quot;0&quot;}')

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-34.383353&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;2&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-58.750027&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nivel4Abreviada15&quot;,&quot;value&quot;:&quot;ING MASCHWITCHZ&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-58.750027&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-34.383353&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;2&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
