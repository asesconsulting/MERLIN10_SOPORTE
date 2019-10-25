<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_TarjetaNaranja</name>
   <tag></tag>
   <elementGuidId>e6c32030-7ac8-49e5-b91d-8e2d391c492d</elementGuidId>
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
   <restUrl>${AddressJson}?level1=AR&amp;street=AVENIDA%20NORBERTO%20DE%20LA%20RIESTRA%201820&amp;level4=CABA&amp;clientAccessCode=c0c40276f6fe778cc08a156d04a79eb4</restUrl>
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
WS.verifyElementPropertyValue(response, 'nAddress.level5', &quot;FLORES&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.streetType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.street', &quot;AVDA NORBERTO DE LA RIESTRA&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.houseNumber', &quot;1820&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.floor', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.unit', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.latitude', &quot;-34.646383&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.longitude', &quot;-58.435269&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.postalCode', &quot;1437&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalPostalCode', &quot;C1437HIU&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.fromStreetNumber', &quot;1101&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.toStreetNumber', &quot;6350&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.additionalData', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet1', &quot;PRES CAMILO TORRES Y TENORIO&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.betweenStreet2', &quot;AGUSTIN DE VEDIA&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.corner', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeType', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.place', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.placeReference', &quot;&quot;)
WS.verifyElementPropertyValue(response, 'nAddress.numberAlternativeAddresses', &quot;0&quot;)
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;localidadAbreviada&quot;,&quot;value&quot;:&quot;CAPITAL FEDERAL&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;calleAbreviada&quot;,&quot;value&quot;:&quot;NORBERTO LA RIESTRA&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;cpa&quot;,&quot;value&quot;:&quot;C1437HIU&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;S&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;NISE&quot;,&quot;value&quot;:&quot;3&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaRiesgo&quot;,&quot;value&quot;:&quot;S&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Longitude&quot;,&quot;value&quot;:&quot;-58.445288&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nise&quot;,&quot;value&quot;:&quot;3&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;inMaq&quot;,&quot;value&quot;:&quot;0&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqValue&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqConcept&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;latitudLocalidad&quot;,&quot;value&quot;:&quot;-34.607161&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;puerta&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;maqStatus&quot;,&quot;value&quot;:&quot;NO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;merlinRiskArea&quot;,&quot;value&quot;:&quot;CAPITAL 3&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;nivelRiesgo&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;barrioAbreviado&quot;,&quot;value&quot;:&quot;FLORES&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;longitudLocalidad&quot;,&quot;value&quot;:&quot;-58.445288&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;zonaRiesgo&quot;,&quot;value&quot;:&quot;CAPITAL 3&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;concepto&quot;,&quot;value&quot;:&quot;&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;postalCertifiedAddresses&quot;,&quot;value&quot;:&quot;NO RELEVADO&quot;}')
assertThat(response.getResponseText()).contains('{&quot;name&quot;:&quot;level4Latitude&quot;,&quot;value&quot;:&quot;-34.607161&quot;}')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
