<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO</name>
   <tag></tag>
   <elementGuidId>2cc2398b-cfc6-4aa4-86aa-9fe99002e61c</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=San%20Martin%20500&amp;level4=CABA&amp;clientAccessCode=aea243aba41084aa098b3a70eeb63ddf</restUrl>
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
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;SAN NICOLAS&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;SAN MARTIN&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;500&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-34.602308&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-58.373966&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;1004&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;C1004AAJ&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;1&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;1400&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;AVDA CORRIENTES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;GRAL JUAN LAVALLE&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;0&quot;)
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;puerta&quot;,&quot;value&quot;:&quot;SI&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaRiesgo&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;8&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-58.445288&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;8&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;inMaq&quot;,&quot;value&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqValue&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqConcept&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-34.607161&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;N&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqStatus&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nivelRiesgo&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-58.445288&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;tipoVivienda&quot;,&quot;value&quot;:&quot;otro&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;concepto&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;postalCertifiedAddresses&quot;,&quot;value&quot;:&quot;SI&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;empresa&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;cpa&quot;,&quot;value&quot;:&quot;C1004AAJ&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-34.607161&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
