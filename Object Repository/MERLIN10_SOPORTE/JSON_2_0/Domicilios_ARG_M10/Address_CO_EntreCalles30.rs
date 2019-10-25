<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_EntreCalles30</name>
   <tag></tag>
   <elementGuidId>a420d48f-899c-4a4b-87ef-d29a1b65605d</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=AVENIDA%20COSTANERA%20DOCTOR%20TRISTAN%20ACHAVAL%20RODRIGUEZ%201508&amp;level4=caba&amp;clientAccessCode=a1edeae2a5bd4cde241fdfdb193ca13c</restUrl>
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


WS.verifyElementPropertyValue(response, 'status', &quot;CO&quot;)
WS.verifyElementPropertyValue(response, 'statusReason', &quot;SM&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.geoType', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level1', &quot;AR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level2', &quot;CAPITAL FEDERAL&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level3', &quot;CAPITAL FEDERAL&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level4', &quot;CIUDAD AUTONOMA BUENOS AIRES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;PUERTO MADERO&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;AVDA COSTANERA DR TRISTAN ACHAVAL RODRIGUEZ&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;1508&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-34.617240&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-58.356545&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;1107&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;C1107ACR&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;1101&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;2200&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;DRA ELVIRA RAWSON DE DELLEPIANE&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;PADRE MARIO LUIS MIGONE&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;0&quot;)

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqStatus&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;6&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;postalCertifiedAddresses&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-58.445288&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqValue&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqConcept&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-34.607161&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;N&quot;}')

assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;beteewStreet2Abbreviated24&quot;,&quot;value&quot;:&quot;PADRE MARIO LUIS MIGONE&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonallevel5abbreviated12&quot;,&quot;value&quot;:&quot;PTO MADERO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4abbreviated15&quot;,&quot;value&quot;:&quot;CAPITAL FEDERAL&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;abbreviatedStreet30&quot;,&quot;value&quot;:&quot;AVENIDA ACHAVAL RODRIGUEZ&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;beteewStreet1Abbreviated24&quot;,&quot;value&quot;:&quot;DRA ELVIRA RAWSON DE DELLEPIANE&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
